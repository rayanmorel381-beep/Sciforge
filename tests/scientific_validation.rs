use sciforge_hub::prelude::*;
use sciforge_hub::tools::validation::report_to_tsv;
use std::fs;
use std::path::Path;

fn run(domain: DomainType, function: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(domain, function);
    for (key, value) in params {
        exp = exp.param(key, value);
    }
    ExperimentRunner::new()
        .run(&exp)
        .unwrap_or_else(|_| panic!("dispatch '{function}' failed"))
}

fn scalar(output: RunOutput) -> f64 {
    match output {
        RunOutput::Scalar(value) => value,
        other => panic!("expected scalar output, got {other:?}"),
    }
}

fn validation_cases() -> Vec<ValidationCase> {
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
            "geology_half_life_from_lambda",
            DomainType::Geology,
            "half_life",
            vec![("lambda", std::f64::consts::LN_2 / 5730.0)],
            5730.0,
            1e-12,
            "Half-life definition",
        ),
        ValidationCase::new(
            "meteorology_relative_humidity_ratio",
            DomainType::Meteorology,
            "relative_humidity",
            vec![("e", 10.0), ("es", 20.0)],
            50.0,
            1e-12,
            "Relative humidity percentage definition",
        ),
        ValidationCase::new(
            "maths_uniform_cdf_midpoint",
            DomainType::Maths,
            "prob_uniform_cdf",
            vec![("x", 0.5), ("a", 0.0), ("b", 1.0)],
            0.5,
            1e-12,
            "Uniform CDF on [0,1]",
        ),
    ]
}

fn ensure_validation_artifacts(report: &ValidationReport) {
    let dir = Path::new("output/tests/validation");
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join("scientific_validation.md"), report.to_markdown()).unwrap();
    fs::write(dir.join("scientific_validation.csv"), report.to_csv()).unwrap();
    fs::write(dir.join("scientific_validation.tsv"), report_to_tsv(report)).unwrap();
    assert!(dir.join("scientific_validation.md").exists());
    assert!(dir.join("scientific_validation.csv").exists());
    assert!(dir.join("scientific_validation.tsv").exists());
}

#[test]
fn scientific_reference_validation_passes() {
    let cases = validation_cases();
    let report = run_validation(&cases);
    assert!(
        report.all_passed(),
        "reference validation failures: {:?}",
        report.failures()
    );
    assert_eq!(report.total(), cases.len());
    ensure_validation_artifacts(&report);
}

#[test]
fn constants_are_consistent_with_public_domain_exports() {
    const { assert!(constants::C == 299_792_458.0) };
    const { assert!(constants::G > 0.0) };
    const { assert!(constants::N_A > 6.0e23 - 1.0e20) };
    const { assert!((constants::ATM_TO_PASCAL - 101_325.0).abs() < 1e-12) };
    const { assert!(constants::EARTH_RADIUS > 6.0e6) };
    const { assert!(constants::COMPTON_WAVELENGTH > 0.0) };
}

#[test]
fn systematic_edge_cases_remain_finite() {
    let checks = vec![
        (
            DomainType::Physics,
            "time_dilation",
            vec![
                ("proper_time", ParameterValue::Scalar(0.0)),
                ("v", ParameterValue::Scalar(0.0)),
            ],
        ),
        (
            DomainType::Chemistry,
            "ideal_gas_pressure",
            vec![
                ("n", ParameterValue::Scalar(1.0)),
                ("t", ParameterValue::Scalar(273.15)),
                ("v", ParameterValue::Scalar(22.4)),
            ],
        ),
        (
            DomainType::Biology,
            "michaelis_menten",
            vec![
                ("s", ParameterValue::Scalar(0.0)),
                ("vmax", ParameterValue::Scalar(100.0)),
                ("km", ParameterValue::Scalar(5.0)),
            ],
        ),
        (
            DomainType::Astronomy,
            "kepler_period",
            vec![
                ("a", ParameterValue::Scalar(7.0e6)),
                ("mu", ParameterValue::Scalar(3.986e14)),
            ],
        ),
        (
            DomainType::Geology,
            "radioactive_decay",
            vec![
                ("n0", ParameterValue::Scalar(1.0)),
                ("lambda", ParameterValue::Scalar(1e-6)),
                ("t", ParameterValue::Scalar(0.0)),
            ],
        ),
        (
            DomainType::Meteorology,
            "barometric_formula",
            vec![
                ("p0", ParameterValue::Scalar(101_325.0)),
                ("m", ParameterValue::Scalar(0.028_964_4)),
                ("g", ParameterValue::Scalar(9.806_65)),
                ("h", ParameterValue::Scalar(0.0)),
                ("t", ParameterValue::Scalar(288.15)),
            ],
        ),
        (
            DomainType::Maths,
            "prob_normal_pdf",
            vec![
                ("x", ParameterValue::Scalar(0.0)),
                ("mu", ParameterValue::Scalar(0.0)),
                ("sigma", ParameterValue::Scalar(1.0)),
            ],
        ),
    ];

    for (domain, function, params) in checks {
        let value = scalar(run(domain, function, params));
        assert!(
            value.is_finite(),
            "{function} should stay finite on edge-case input"
        );
    }
}
