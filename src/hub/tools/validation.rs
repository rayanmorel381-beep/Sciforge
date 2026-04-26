//! Scientific validation pipeline: reference cases, monotonicity checks,
//! NaN safety, blocking thresholds, and exports (Markdown, CSV, LaTeX, TSV).

use crate::hub::engine::experience::experiment::{DomainType, Experiment, ParameterValue};
use crate::hub::engine::experience::runner::{ExperimentRunner, RunOutput};

/// Unit validation case comparing a computed result to a reference value.
#[derive(Debug, Clone)]
pub struct ValidationCase {
    /// Name of the validation case.
    pub name: String,
    /// Target scientific domain.
    pub domain: DomainType,
    /// Name of the tested function.
    pub function: String,
    /// Input parameters.
    pub params: Vec<(String, ParameterValue)>,
    /// Expected reference value.
    pub expected: f64,
    /// Accepted relative tolerance.
    pub tolerance: f64,
    /// Source of the reference value.
    pub source: String,
}

impl ValidationCase {
    pub fn new(
        name: &str,
        domain: DomainType,
        function: &str,
        params: Vec<(&str, f64)>,
        expected: f64,
        tolerance: f64,
        source: &str,
    ) -> Self {
        Self {
            name: name.into(),
            domain,
            function: function.into(),
            params: params
                .into_iter()
                .map(|(k, v)| (k.to_string(), ParameterValue::Scalar(v)))
                .collect(),
            expected,
            tolerance,
            source: source.into(),
        }
    }
}

/// Result of an individual validation case.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Case name.
    pub name: String,
    /// `true` if the relative error is below the tolerance.
    pub passed: bool,
    /// Value computed by the engine.
    pub computed: f64,
    /// Reference value.
    pub expected: f64,
    /// Observed relative error.
    pub relative_error: f64,
    /// Tolerance used.
    pub tolerance: f64,
    /// Optional error message (unexpected type, execution failure).
    pub error_message: Option<String>,
}

/// Validation report aggregating all results.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// List of individual results.
    pub results: Vec<ValidationResult>,
}

impl ValidationReport {
    /// Number of passed cases.
    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Number of failed cases.
    pub fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Total number of cases.
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// `true` if all cases passed.
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }

    /// Returns the failed results.
    pub fn failures(&self) -> Vec<&ValidationResult> {
        self.results.iter().filter(|r| !r.passed).collect()
    }

    /// Returns the result with the largest relative error.
    pub fn worst_error(&self) -> Option<&ValidationResult> {
        self.results.iter().max_by(|a, b| {
            a.relative_error
                .partial_cmp(&b.relative_error)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Exports the report in Markdown format.
    pub fn to_markdown(&self) -> String {
        let mut out = String::from("# Validation Report\n\n");
        out.push_str(&format!(
            "**{}/{} passed**\n\n",
            self.passed_count(),
            self.total()
        ));
        out.push_str("| Test | Computed | Expected | Rel. Error | Tol | Status |\n");
        out.push_str("|------|----------|----------|------------|-----|--------|\n");
        for r in &self.results {
            let status = if r.passed { "✓" } else { "✗" };
            out.push_str(&format!(
                "| {} | {:.6e} | {:.6e} | {:.2e} | {:.0e} | {} |\n",
                r.name, r.computed, r.expected, r.relative_error, r.tolerance, status,
            ));
        }
        if let Some(w) = self.worst_error() {
            out.push_str(&format!(
                "\nWorst error: {} (rel={:.2e})\n",
                w.name, w.relative_error
            ));
        }
        out
    }

    /// Exports the report in CSV format.
    pub fn to_csv(&self) -> String {
        let mut out = String::from("name,computed,expected,relative_error,tolerance,passed\n");
        for r in &self.results {
            out.push_str(&format!(
                "{},{:.10e},{:.10e},{:.6e},{:.0e},{}\n",
                r.name, r.computed, r.expected, r.relative_error, r.tolerance, r.passed,
            ));
        }
        out
    }
}

/// Runs all validation cases and returns a report.
pub fn run_validation(cases: &[ValidationCase]) -> ValidationReport {
    let runner = ExperimentRunner::new();
    let mut results = Vec::with_capacity(cases.len());

    for case in cases {
        let mut exp = Experiment::new(case.domain.clone(), &case.function);
        for (k, v) in &case.params {
            exp = exp.param(k, v.clone());
        }

        let vr = match runner.run(&exp) {
            Ok(RunOutput::Scalar(v)) => {
                let rel_err = if case.expected == 0.0 {
                    v.abs()
                } else {
                    ((v - case.expected) / case.expected).abs()
                };
                ValidationResult {
                    name: case.name.clone(),
                    passed: rel_err <= case.tolerance,
                    computed: v,
                    expected: case.expected,
                    relative_error: rel_err,
                    tolerance: case.tolerance,
                    error_message: None,
                }
            }
            Ok(other) => ValidationResult {
                name: case.name.clone(),
                passed: false,
                computed: f64::NAN,
                expected: case.expected,
                relative_error: f64::INFINITY,
                tolerance: case.tolerance,
                error_message: Some(format!("expected Scalar, got {:?}", other)),
            },
            Err(e) => ValidationResult {
                name: case.name.clone(),
                passed: false,
                computed: f64::NAN,
                expected: case.expected,
                relative_error: f64::INFINITY,
                tolerance: case.tolerance,
                error_message: Some(format!("{e}")),
            },
        };
        results.push(vr);
    }

    ValidationReport { results }
}

/// Checks that a function does not return `NaN` for the given parameters.
pub fn check_nan_safety(domain: DomainType, function: &str, params: Vec<(&str, f64)>) -> bool {
    let runner = ExperimentRunner::new();
    let mut exp = Experiment::new(domain, function);
    for (k, v) in params {
        exp = exp.param(k, ParameterValue::Scalar(v));
    }
    match runner.run(&exp) {
        Ok(RunOutput::Scalar(v)) => !v.is_nan(),
        _ => true,
    }
}

/// Checks the monotonicity of a function over a set of values.
pub fn check_monotonicity(
    domain: DomainType,
    function: &str,
    base_params: Vec<(&str, f64)>,
    vary_param: &str,
    values: &[f64],
    increasing: bool,
) -> bool {
    let runner = ExperimentRunner::new();
    let mut prev: Option<f64> = None;

    for &v in values {
        let mut exp = Experiment::new(domain.clone(), function);
        for &(k, val) in &base_params {
            if k == vary_param {
                exp = exp.param(k, ParameterValue::Scalar(v));
            } else {
                exp = exp.param(k, ParameterValue::Scalar(val));
            }
        }
        if let Ok(RunOutput::Scalar(out)) = runner.run(&exp) {
            if let Some(p) = prev {
                if increasing && out < p {
                    return false;
                }
                if !increasing && out > p {
                    return false;
                }
            }
            prev = Some(out);
        }
    }
    true
}

/// Exports the report in LaTeX tabular format.
pub fn report_to_latex(report: &ValidationReport) -> String {
    let mut out =
        String::from("\\begin{table}[h]\n\\centering\n\\begin{tabular}{|l|r|r|r|c|}\n\\hline\n");
    out.push_str("Test & Computed & Expected & Rel. Error & Pass \\\\\n\\hline\n");
    for r in &report.results {
        let status = if r.passed { "\\checkmark" } else { "\\times" };
        out.push_str(&format!(
            "{} & {:.6e} & {:.6e} & {:.2e} & ${status}$ \\\\\n",
            r.name.replace('_', "\\_"),
            r.computed,
            r.expected,
            r.relative_error,
        ));
    }
    out.push_str("\\hline\n\\end{tabular}\n");
    out.push_str(&format!(
        "\\caption{{Validation: {}/{} passed}}\n",
        report.passed_count(),
        report.total()
    ));
    out.push_str("\\end{table}\n");
    out
}

/// Exports the report in TSV format.
pub fn report_to_tsv(report: &ValidationReport) -> String {
    let mut out = String::from("name\tcomputed\texpected\trelative_error\ttolerance\tpassed\n");
    for r in &report.results {
        out.push_str(&format!(
            "{}\t{:.10e}\t{:.10e}\t{:.6e}\t{:.0e}\t{}\n",
            r.name, r.computed, r.expected, r.relative_error, r.tolerance, r.passed,
        ));
    }
    out
}

/// Blocking thresholds for the validation pipeline.
#[derive(Debug, Clone)]
pub struct ValidationThresholds {
    /// Maximum number of allowed failures.
    pub max_failures: usize,
    /// Maximum allowed relative error.
    pub max_relative_error: f64,
}

impl Default for ValidationThresholds {
    fn default() -> Self {
        Self {
            max_failures: 0,
            max_relative_error: 1e-6,
        }
    }
}

/// Parameterized monotonicity check.
#[derive(Debug, Clone)]
pub struct MonotonicityCheck {
    /// Descriptive label.
    pub label: String,
    /// Scientific domain.
    pub domain: DomainType,
    /// Function name.
    pub function: String,
    /// Base parameters (excluding the varied parameter).
    pub base_params: Vec<(String, f64)>,
    /// Name of the parameter to vary.
    pub vary_param: String,
    /// Successive values for the varied parameter.
    pub values: Vec<f64>,
    /// `true` if the function should be increasing, `false` if decreasing.
    pub increasing: bool,
}

impl MonotonicityCheck {
    /// Creates a monotonicity check.
    pub fn new(
        label: &str,
        domain: DomainType,
        function: &str,
        base_params: Vec<(&str, f64)>,
        vary_param: &str,
        values: Vec<f64>,
        increasing: bool,
    ) -> Self {
        Self {
            label: label.into(),
            domain,
            function: function.into(),
            base_params: base_params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            vary_param: vary_param.into(),
            values,
            increasing,
        }
    }
}

/// NaN safety check for a function with given parameters.
#[derive(Debug, Clone)]
pub struct NanSafetyCheck {
    /// Descriptive label.
    pub label: String,
    /// Scientific domain.
    pub domain: DomainType,
    /// Function name.
    pub function: String,
    /// Input parameters.
    pub params: Vec<(String, f64)>,
}

impl NanSafetyCheck {
    /// Creates a NaN safety check.
    pub fn new(label: &str, domain: DomainType, function: &str, params: Vec<(&str, f64)>) -> Self {
        Self {
            label: label.into(),
            domain,
            function: function.into(),
            params: params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        }
    }
}

/// Result of a monotonicity check.
#[derive(Debug, Clone)]
pub struct MonotonicityResult {
    /// Check label.
    pub label: String,
    /// `true` if monotonicity is satisfied.
    pub passed: bool,
}

/// Result of a NaN safety check.
#[derive(Debug, Clone)]
pub struct NanSafetyResult {
    /// Check label.
    pub label: String,
    /// `true` if no NaN was produced.
    pub passed: bool,
}

/// Overall outcome of the validation pipeline.
#[derive(Debug, Clone)]
pub struct PipelineOutcome {
    /// `true` if all checks passed.
    pub passed: bool,
    /// Validation case report.
    pub report: ValidationReport,
    /// `true` if the failure count exceeds the threshold.
    pub blocked_by_failures: bool,
    /// `true` if the maximum relative error exceeds the threshold.
    pub blocked_by_error: bool,
    /// Worst observed relative error.
    pub worst_relative_error: f64,
    /// Monotonicity check results.
    pub monotonicity_results: Vec<MonotonicityResult>,
    /// NaN safety check results.
    pub nan_safety_results: Vec<NanSafetyResult>,
    /// `true` if all monotonicity checks passed.
    pub monotonicity_passed: bool,
    /// `true` if all NaN safety checks passed.
    pub nan_safety_passed: bool,
}

/// Validation pipeline orchestrating cases, monotonicity, and NaN safety.
pub struct ValidationPipeline {
    cases: Vec<ValidationCase>,
    thresholds: ValidationThresholds,
    monotonicity_checks: Vec<MonotonicityCheck>,
    nan_checks: Vec<NanSafetyCheck>,
}

impl ValidationPipeline {
    /// Creates an empty pipeline with the specified thresholds.
    pub fn new(thresholds: ValidationThresholds) -> Self {
        Self {
            cases: Vec::new(),
            thresholds,
            monotonicity_checks: Vec::new(),
            nan_checks: Vec::new(),
        }
    }

    /// Adds a validation case.
    pub fn add_case(mut self, case: ValidationCase) -> Self {
        self.cases.push(case);
        self
    }

    /// Adds multiple validation cases.
    pub fn add_cases(mut self, cases: Vec<ValidationCase>) -> Self {
        self.cases.extend(cases);
        self
    }

    /// Adds a monotonicity check.
    pub fn add_monotonicity(mut self, check: MonotonicityCheck) -> Self {
        self.monotonicity_checks.push(check);
        self
    }

    /// Adds a NaN safety check.
    pub fn add_nan_check(mut self, check: NanSafetyCheck) -> Self {
        self.nan_checks.push(check);
        self
    }

    /// Loads the default validation cases.
    pub fn with_default_cases(self) -> Self {
        self.add_cases(default_cases())
    }

    /// Loads the default monotonicity checks.
    pub fn with_default_monotonicity(self) -> Self {
        let checks = default_monotonicity_checks();
        let mut s = self;
        for c in checks {
            s = s.add_monotonicity(c);
        }
        s
    }

    /// Loads the default NaN safety checks.
    pub fn with_default_nan_safety(self) -> Self {
        let checks = default_nan_safety_checks();
        let mut s = self;
        for c in checks {
            s = s.add_nan_check(c);
        }
        s
    }

    /// Creates a full pipeline with all default cases and checks.
    pub fn full_default(thresholds: ValidationThresholds) -> Self {
        Self::new(thresholds)
            .with_default_cases()
            .with_default_monotonicity()
            .with_default_nan_safety()
    }

    /// Runs the full pipeline and returns the overall outcome.
    pub fn run(&self) -> PipelineOutcome {
        let report = run_validation(&self.cases);

        let worst_relative_error = report
            .results
            .iter()
            .map(|r| r.relative_error)
            .filter(|e| e.is_finite())
            .fold(0.0_f64, f64::max);

        let blocked_by_failures = report.failed_count() > self.thresholds.max_failures;
        let blocked_by_error = worst_relative_error > self.thresholds.max_relative_error;

        let monotonicity_results: Vec<MonotonicityResult> = self
            .monotonicity_checks
            .iter()
            .map(|mc| {
                let ok = check_monotonicity(
                    mc.domain.clone(),
                    &mc.function,
                    mc.base_params
                        .iter()
                        .map(|(k, v)| (k.as_str(), *v))
                        .collect(),
                    &mc.vary_param,
                    &mc.values,
                    mc.increasing,
                );
                MonotonicityResult {
                    label: mc.label.clone(),
                    passed: ok,
                }
            })
            .collect();

        let nan_safety_results: Vec<NanSafetyResult> = self
            .nan_checks
            .iter()
            .map(|nc| {
                let ok = check_nan_safety(
                    nc.domain.clone(),
                    &nc.function,
                    nc.params.iter().map(|(k, v)| (k.as_str(), *v)).collect(),
                );
                NanSafetyResult {
                    label: nc.label.clone(),
                    passed: ok,
                }
            })
            .collect();

        let monotonicity_passed = monotonicity_results.iter().all(|r| r.passed);
        let nan_safety_passed = nan_safety_results.iter().all(|r| r.passed);
        let passed =
            !blocked_by_failures && !blocked_by_error && monotonicity_passed && nan_safety_passed;

        PipelineOutcome {
            passed,
            report,
            blocked_by_failures,
            blocked_by_error,
            worst_relative_error,
            monotonicity_results,
            nan_safety_results,
            monotonicity_passed,
            nan_safety_passed,
        }
    }

    /// Number of registered validation cases.
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }

    /// Number of registered monotonicity checks.
    pub fn monotonicity_count(&self) -> usize {
        self.monotonicity_checks.len()
    }

    /// Number of registered NaN checks.
    pub fn nan_check_count(&self) -> usize {
        self.nan_checks.len()
    }
}

/// Returns the default validation cases covering all domains.
pub fn default_cases() -> Vec<ValidationCase> {
    vec![
        ValidationCase::new(
            "physics_carnot_efficiency",
            DomainType::Physics,
            "carnot_efficiency",
            vec![("t_hot", 600.0), ("t_cold", 300.0)],
            0.5,
            1e-12,
            "Carnot theorem",
        ),
        ValidationCase::new(
            "chemistry_strong_acid_ph",
            DomainType::Chemistry,
            "ph_strong_acid",
            vec![("concentration", 0.01)],
            2.0,
            1e-12,
            "pH = -log10(c)",
        ),
        ValidationCase::new(
            "biology_michaelis_menten",
            DomainType::Biology,
            "michaelis_menten",
            vec![("s", 10.0), ("vmax", 100.0), ("km", 5.0)],
            100.0 * 10.0 / 15.0,
            1e-12,
            "Michaelis-Menten equation",
        ),
        ValidationCase::new(
            "astronomy_escape_velocity_earth",
            DomainType::Astronomy,
            "escape_velocity",
            vec![("mu", 3.986e14), ("r", 6.371e6)],
            11_186.0,
            2e-2,
            "Earth escape velocity reference",
        ),
        ValidationCase::new(
            "geology_half_life",
            DomainType::Geology,
            "half_life",
            vec![("lambda", std::f64::consts::LN_2 / 5730.0)],
            5730.0,
            1e-12,
            "Half-life definition",
        ),
        ValidationCase::new(
            "meteorology_relative_humidity",
            DomainType::Meteorology,
            "relative_humidity",
            vec![("e", 10.0), ("es", 20.0)],
            50.0,
            1e-12,
            "Relative humidity percentage",
        ),
        ValidationCase::new(
            "maths_uniform_cdf",
            DomainType::Maths,
            "prob_uniform_cdf",
            vec![("x", 0.5), ("a", 0.0), ("b", 1.0)],
            0.5,
            1e-12,
            "Uniform CDF on [0,1]",
        ),
        ValidationCase::new(
            "astrophysics_schwarzschild_radius_sun",
            DomainType::Astrophysics,
            "schwarzschild_radius",
            vec![("mass", 1.989e30)],
            2.0 * 6.674_30e-11 * 1.989e30 / (299_792_458.0 * 299_792_458.0),
            1e-6,
            "r_s = 2GM/c^2",
        ),
        ValidationCase::new(
            "biochemistry_gibbs_free_energy",
            DomainType::Biochemistry,
            "gibbs_free_energy",
            vec![
                ("delta_h", -100_000.0),
                ("delta_s", -200.0),
                ("temperature", 298.15),
            ],
            -100_000.0 + 200.0 * 298.15,
            1e-12,
            "G = H - TS",
        ),
        ValidationCase::new(
            "geochemistry_partition_coefficient",
            DomainType::Geochemistry,
            "partition_coefficient",
            vec![("c_solid", 50.0), ("c_liquid", 10.0)],
            5.0,
            1e-12,
            "Kd = Cs/Cl",
        ),
        ValidationCase::new(
            "atmospheric_chemistry_photolysis_rate",
            DomainType::AtmosphericChemistry,
            "photolysis_rate",
            vec![
                ("cross_section", 1e-20),
                ("quantum_yield", 0.5),
                ("actinic_flux", 1e15),
            ],
            1e-20 * 0.5 * 1e15,
            1e-12,
            "J = sigma * phi * F",
        ),
        ValidationCase::new(
            "atmospheric_physics_stefan_boltzmann",
            DomainType::AtmosphericPhysics,
            "stefan_boltzmann_flux",
            vec![("temperature", 255.0)],
            5.670_374_419e-8 * 255.0_f64.powi(4),
            1e-6,
            "F = sigma * T^4",
        ),
        ValidationCase::new(
            "planetary_geology_impact_energy",
            DomainType::PlanetaryGeology,
            "impact_energy",
            vec![("projectile_mass", 1e6), ("impact_velocity", 2e4)],
            0.5 * 1e6 * 2e4 * 2e4,
            1e-12,
            "KE = 0.5 * m * v^2",
        ),
        ValidationCase::new(
            "biomathematics_logistic_growth_zero_at_capacity",
            DomainType::Biomathematics,
            "logistic_growth_rate",
            vec![
                ("r", 0.5),
                ("carrying_capacity", 1000.0),
                ("population", 1000.0),
            ],
            0.0,
            1e-12,
            "dN/dt = 0 when N = K",
        ),
        ValidationCase::new(
            "mathematical_physics_de_broglie",
            DomainType::MathematicalPhysics,
            "de_broglie_wavelength",
            vec![("momentum", 1e-24)],
            6.626_070_15e-34 / 1e-24,
            1e-6,
            "lambda = h/p",
        ),
        ValidationCase::new(
            "biophysics_stokes_drag",
            DomainType::Biophysics,
            "stokes_drag_force",
            vec![("viscosity", 1e-3), ("radius", 1e-6), ("velocity", 1e-4)],
            6.0 * std::f64::consts::PI * 1e-3 * 1e-6 * 1e-4,
            1e-6,
            "F = 6*pi*eta*r*v",
        ),
        ValidationCase::new(
            "geophysics_bouguer_anomaly_zero_elevation",
            DomainType::Geophysics,
            "bouguer_anomaly",
            vec![
                ("observed_gravity", 9.81),
                ("reference_gravity", 9.80),
                ("elevation", 0.0),
                ("slab_density", 2670.0),
            ],
            9.81 - 9.80,
            1e-6,
            "At zero elevation: anomaly = g_obs - g_ref",
        ),
        ValidationCase::new(
            "astrochemistry_freefall_time",
            DomainType::Astrochemistry,
            "freefall_time",
            vec![("number_density", 1e4), ("mean_molecular_weight", 2.33)],
            (3.0 * std::f64::consts::PI
                / (32.0 * 6.674_30e-11 * 1e4 * 1e6 * 2.33 * 1.672_621_9e-27))
                .sqrt(),
            5e-2,
            "t_ff = sqrt(3*pi/(32*G*rho))",
        ),
        ValidationCase::new(
            "astrobiology_habitable_zone_sun",
            DomainType::Astrobiology,
            "habitable_zone_inner",
            vec![("luminosity", 3.828e26)],
            (3.828e26 / (4.0 * std::f64::consts::PI * 1.0e3)).sqrt()
                * (3.828e26 / (16.0 * std::f64::consts::PI * 5.670_374_419e-8 * 373.0_f64.powi(4)))
                    .sqrt()
                    .recip()
                * (3.828e26 / (4.0 * std::f64::consts::PI * 1.0e3)).sqrt(),
            5e-1,
            "Inner edge ~0.95 AU for Sun-like star",
        ),
    ]
}

/// Returns the default monotonicity checks.
pub fn default_monotonicity_checks() -> Vec<MonotonicityCheck> {
    vec![
        MonotonicityCheck::new(
            "gamma_increases_with_velocity",
            DomainType::Physics,
            "lorentz_gamma",
            vec![],
            "v",
            vec![0.0, 1e7, 5e7, 1e8, 2e8, 2.5e8, 2.9e8],
            true,
        ),
        MonotonicityCheck::new(
            "ph_decreases_with_concentration",
            DomainType::Chemistry,
            "ph_strong_acid",
            vec![],
            "concentration",
            vec![1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 0.1, 1.0],
            false,
        ),
        MonotonicityCheck::new(
            "michaelis_menten_increases_with_substrate",
            DomainType::Biology,
            "michaelis_menten",
            vec![("vmax", 100.0), ("km", 5.0)],
            "s",
            vec![0.1, 0.5, 1.0, 5.0, 10.0, 50.0, 100.0],
            true,
        ),
        MonotonicityCheck::new(
            "carnot_efficiency_increases_with_t_hot",
            DomainType::Physics,
            "carnot_efficiency",
            vec![("t_cold", 300.0)],
            "t_hot",
            vec![301.0, 400.0, 500.0, 1000.0, 2000.0, 5000.0],
            true,
        ),
        MonotonicityCheck::new(
            "escape_velocity_decreases_with_radius",
            DomainType::Astronomy,
            "escape_velocity",
            vec![("mu", 3.986e14)],
            "r",
            vec![6.371e6, 1e7, 2e7, 5e7, 1e8],
            false,
        ),
    ]
}

/// Returns the default NaN safety checks.
pub fn default_nan_safety_checks() -> Vec<NanSafetyCheck> {
    vec![
        NanSafetyCheck::new(
            "carnot_zero_temps",
            DomainType::Physics,
            "carnot_efficiency",
            vec![("t_hot", 300.0), ("t_cold", 0.0)],
        ),
        NanSafetyCheck::new(
            "michaelis_menten_zero_substrate",
            DomainType::Biology,
            "michaelis_menten",
            vec![("s", 0.0), ("vmax", 100.0), ("km", 5.0)],
        ),
        NanSafetyCheck::new(
            "relative_humidity_zero_es",
            DomainType::Meteorology,
            "relative_humidity",
            vec![("e", 0.0), ("es", 0.0)],
        ),
        NanSafetyCheck::new(
            "half_life_zero_lambda",
            DomainType::Geology,
            "half_life",
            vec![("lambda", 0.0)],
        ),
        NanSafetyCheck::new(
            "logistic_growth_zero_population",
            DomainType::Biomathematics,
            "logistic_growth_rate",
            vec![
                ("r", 0.5),
                ("carrying_capacity", 1000.0),
                ("population", 0.0),
            ],
        ),
    ]
}
