pub fn drug_drug_interaction_auc_ratio(inhibitor_conc: f64, ki: f64) -> f64 {
    1.0 + inhibitor_conc / ki
}

pub fn cyp_induction_fold(inducer_conc: f64, ec50: f64, emax: f64) -> f64 {
    1.0 + emax * inducer_conc / (ec50 + inducer_conc)
}

pub fn competitive_displacement(drug_a_bound: f64, drug_b_conc: f64, kb: f64) -> f64 {
    drug_a_bound / (1.0 + drug_b_conc / kb)
}

pub fn synergy_bliss_independence(effect_a: f64, effect_b: f64) -> f64 {
    effect_a + effect_b - effect_a * effect_b
}

pub fn loewe_combination_index(
    dose_a: f64,
    dose_a_alone: f64,
    dose_b: f64,
    dose_b_alone: f64,
) -> f64 {
    dose_a / dose_a_alone.max(1e-30) + dose_b / dose_b_alone.max(1e-30)
}

pub fn isobologram_point(dose_a: f64, ic50_a: f64, dose_b: f64, ic50_b: f64) -> f64 {
    dose_a / ic50_a + dose_b / ic50_b
}

pub fn prodrug_activation(
    prodrug_conc: f64,
    enzyme_activity: f64,
    km: f64,
    activation_fraction: f64,
) -> f64 {
    activation_fraction * enzyme_activity * prodrug_conc / (km + prodrug_conc)
}

pub fn drug_therapeutic_index(td50: f64, ed50: f64) -> f64 {
    td50 / ed50.max(1e-30)
}

pub fn loading_dose_calculation(
    target_concentration: f64,
    volume_of_distribution: f64,
    bioavailability: f64,
) -> f64 {
    target_concentration * volume_of_distribution / bioavailability.max(1e-30)
}

pub fn maintenance_dose_calculation(
    target_concentration: f64,
    clearance: f64,
    bioavailability: f64,
    dosing_interval: f64,
) -> f64 {
    target_concentration * clearance * dosing_interval / bioavailability.max(1e-30)
}

pub fn steady_state_accumulation(dose: f64, half_life: f64, dosing_interval: f64) -> f64 {
    dose * (1.0 / (1.0 - (-0.693 * dosing_interval / half_life.max(1e-30)).exp()))
}
