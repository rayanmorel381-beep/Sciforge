pub fn cleavage_timing(stage: u32, base_interval: f64, temperature_factor: f64) -> f64 {
    base_interval * stage as f64 / temperature_factor
}

pub fn blastocyst_cell_count(initial_cells: f64, division_rate: f64, t: f64) -> f64 {
    initial_cells * (division_rate * t).exp()
}

pub fn morphogen_gradient_embryo(source: f64, diffusion: f64, degradation: f64, x: f64) -> f64 {
    source * (-(degradation / diffusion).sqrt() * x).exp()
}

pub fn gastrulation_cell_migration(
    chemotactic_sensitivity: f64,
    gradient: f64,
    random_motility: f64,
) -> f64 {
    chemotactic_sensitivity * gradient + random_motility
}

pub fn somitogenesis_clock(frequency: f64, wavefront_speed: f64, position: f64, t: f64) -> f64 {
    (2.0 * std::f64::consts::PI * (frequency * t - position / wavefront_speed)).sin()
}

pub fn fetal_weight_hadlock(gestational_age_weeks: f64) -> f64 {
    let ga = gestational_age_weeks;
    (0.578 + 0.332 * ga - 0.00354 * ga * ga).exp() * 10.0
}

pub fn placental_transfer_rate(
    maternal_conc: f64,
    fetal_conc: f64,
    permeability: f64,
    surface_area: f64,
) -> f64 {
    permeability * surface_area * (maternal_conc - fetal_conc)
}

pub fn crown_rump_length(gestational_age_weeks: f64) -> f64 {
    -0.0013 * gestational_age_weeks.powi(3) + 0.1302 * gestational_age_weeks.powi(2)
        - 1.0804 * gestational_age_weeks
        + 2.4432
}

pub fn biparietal_diameter(gestational_age_weeks: f64) -> f64 {
    -4.0 + 1.07 * gestational_age_weeks - 0.0092 * gestational_age_weeks.powi(2)
}

pub fn amniotic_fluid_index(quadrants: &[f64; 4]) -> f64 {
    quadrants.iter().sum()
}

pub fn neural_tube_closure_progress(t: f64, rate: f64, max_closure: f64) -> f64 {
    max_closure * (1.0 - (-rate * t).exp())
}

pub fn organogenesis_differentiation_rate(
    morphogen_conc: f64,
    threshold: f64,
    hill_coefficient: f64,
) -> f64 {
    morphogen_conc.powf(hill_coefficient)
        / (threshold.powf(hill_coefficient) + morphogen_conc.powf(hill_coefficient))
}

pub fn turing_activator_inhibitor(
    activator: f64,
    inhibitor: f64,
    rho_a: f64,
    rho_i: f64,
    mu_a: f64,
    mu_i: f64,
    kappa: f64,
) -> (f64, f64) {
    let da = rho_a * activator * activator / inhibitor - mu_a * activator + kappa;
    let di = rho_i * activator * activator - mu_i * inhibitor;
    (da, di)
}

pub fn fetal_lung_maturity_ls_ratio(lecithin: f64, sphingomyelin: f64) -> f64 {
    lecithin / sphingomyelin.max(1e-30)
}

pub fn apgar_component(
    heart_rate: f64,
    respiration: f64,
    muscle_tone: f64,
    reflex: f64,
    color: f64,
) -> f64 {
    heart_rate + respiration + muscle_tone + reflex + color
}

pub fn fetal_heart_rate_baseline(gestational_age_weeks: f64) -> f64 {
    155.0 - 0.7 * gestational_age_weeks
}

pub fn umbilical_artery_pi(systolic: f64, diastolic: f64, mean: f64) -> f64 {
    (systolic - diastolic) / mean.max(1e-30)
}

pub fn placental_oxygen_delivery(
    blood_flow: f64,
    hemoglobin: f64,
    saturation: f64,
    o2_binding_capacity: f64,
) -> f64 {
    blood_flow * hemoglobin * saturation * o2_binding_capacity
}

pub fn trophoblast_invasion_depth(
    migration_rate: f64,
    protease_activity: f64,
    resistance: f64,
    t: f64,
) -> f64 {
    migration_rate * protease_activity / resistance.max(1e-30) * t
}

pub fn gestational_sac_diameter(gestational_age_days: f64) -> f64 {
    gestational_age_days - 30.0
}

pub fn yolk_sac_regression(initial_size: f64, regression_rate: f64, t: f64) -> f64 {
    initial_size * (-regression_rate * t).exp()
}

pub fn limb_bud_outgrowth(fgf_conc: f64, shh_conc: f64, growth_rate: f64, t: f64) -> f64 {
    growth_rate * (fgf_conc * shh_conc).sqrt() * t
}

pub fn cell_fate_probability(signal_strength: f64, noise: f64, threshold: f64) -> f64 {
    1.0 / (1.0 + (-(signal_strength - threshold) / noise.max(1e-30)).exp())
}
