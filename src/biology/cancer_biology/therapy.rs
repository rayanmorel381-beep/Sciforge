pub fn cell_kill_log(initial: f64, surviving_fraction: f64, cycles: u32) -> f64 {
    initial * surviving_fraction.powi(cycles as i32)
}

pub fn skipper_schabel_log_kill(n: f64, dose: f64, sensitivity: f64) -> f64 {
    n * (-sensitivity * dose).exp()
}

pub fn drug_resistance_goldie_coldman(n: f64, mutation_rate: f64) -> f64 {
    1.0 - (-mutation_rate * n).exp()
}

pub fn combination_therapy_survival(sf_a: f64, sf_b: f64, interaction: f64) -> f64 {
    sf_a * sf_b * (1.0 + interaction)
}

pub fn tumor_immune_interaction(
    tumor: f64,
    immune: f64,
    growth_rate: f64,
    kill_rate: f64,
    stimulation: f64,
    decay: f64,
    k: f64,
) -> (f64, f64) {
    let dt = growth_rate * tumor * (1.0 - tumor / k) - kill_rate * immune * tumor;
    let di = stimulation * tumor * immune / (tumor + 1.0) - decay * immune;
    (dt, di)
}

pub fn hallmarks_proliferation_index(mitotic_count: f64, area: f64) -> f64 {
    mitotic_count / area
}

pub fn cancer_stem_cell_fraction(
    symmetric_division_rate: f64,
    asymmetric_division_rate: f64,
    differentiation_rate: f64,
) -> f64 {
    symmetric_division_rate
        / (symmetric_division_rate + asymmetric_division_rate + differentiation_rate)
}

pub fn linear_quadratic_survival(dose: f64, alpha: f64, beta: f64) -> f64 {
    (-(alpha * dose + beta * dose * dose)).exp()
}

pub fn biologically_effective_dose(dose: f64, fractions: f64, alpha_beta: f64) -> f64 {
    dose * (1.0 + dose / (fractions * alpha_beta))
}

pub fn equivalent_dose_2gy(dose: f64, dose_per_fraction: f64, alpha_beta: f64) -> f64 {
    dose * (dose_per_fraction + alpha_beta) / (2.0 + alpha_beta)
}

pub fn tumor_control_probability(n_cells: f64, surviving_fraction: f64) -> f64 {
    (-n_cells * surviving_fraction).exp()
}

pub fn normal_tissue_complication_probability(dose: f64, td50: f64, gamma50: f64) -> f64 {
    let t = (dose - td50) / (gamma50 * td50 / (2.0 * std::f64::consts::PI).sqrt());
    0.5 * (1.0 + (2.0 / std::f64::consts::PI).sqrt() * erf_approx(t / std::f64::consts::SQRT_2))
}

fn erf_approx(x: f64) -> f64 {
    let a = 0.147;
    let x2 = x * x;
    let sign = if x >= 0.0 { 1.0 } else { -1.0 };
    let inner = -x2 * (4.0 / std::f64::consts::PI + a * x2) / (1.0 + a * x2);
    sign * (1.0 - inner.exp()).sqrt()
}

pub fn therapeutic_ratio(tcp: f64, ntcp: f64) -> f64 {
    tcp / (1.0 - ntcp).max(1e-30)
}

pub fn immunotherapy_checkpoint_response(
    tumor: f64,
    t_cells: f64,
    activation_rate: f64,
    exhaustion_rate: f64,
    checkpoint_blockade: f64,
) -> f64 {
    activation_rate * t_cells * (1.0 - exhaustion_rate * (1.0 - checkpoint_blockade)) * tumor
}

pub fn car_t_expansion(
    initial_cells: f64,
    antigen_stimulation: f64,
    expansion_rate: f64,
    t: f64,
) -> f64 {
    initial_cells * (expansion_rate * antigen_stimulation * t).exp()
}

pub fn antibody_drug_conjugate_kill(
    antibody_conc: f64,
    target_density: f64,
    internalization_rate: f64,
    drug_potency: f64,
    kd: f64,
) -> f64 {
    let bound = antibody_conc * target_density / (kd + antibody_conc);
    1.0 - (-internalization_rate * drug_potency * bound).exp()
}

pub fn metronomic_antiangiogenic_effect(dose: f64, frequency: f64, sensitivity: f64) -> f64 {
    1.0 - (-sensitivity * dose * frequency).exp()
}

pub fn fractionation_schedule_bde(
    n_fractions: u32,
    dose_per_fraction: f64,
    alpha_beta: f64,
) -> f64 {
    let total = n_fractions as f64 * dose_per_fraction;
    total * (1.0 + dose_per_fraction / alpha_beta)
}

pub fn cell_cycle_specific_kill(drug_conc: f64, phase_fraction: f64, sensitivity: f64) -> f64 {
    1.0 - (-sensitivity * drug_conc * phase_fraction).exp()
}

pub fn combination_index_chou_talalay(
    fa: f64,
    dose_a: f64,
    dose_b: f64,
    dm_a: f64,
    dm_b: f64,
    m_a: f64,
    m_b: f64,
) -> f64 {
    let dx_a = dm_a * (fa / (1.0 - fa)).powf(1.0 / m_a);
    let dx_b = dm_b * (fa / (1.0 - fa)).powf(1.0 / m_b);
    dose_a / dx_a + dose_b / dx_b
}

pub fn radiation_oxygen_enhancement_ratio(dose_hypoxic: f64, dose_aerobic: f64) -> f64 {
    dose_hypoxic / dose_aerobic.max(1e-30)
}

pub fn hyperthermia_enhancement(dose: f64, thermal_enhancement_ratio: f64) -> f64 {
    dose * thermal_enhancement_ratio
}
