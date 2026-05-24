pub fn antigenic_variation_escape(
    immune_recognition: f64,
    switch_rate: f64,
    variants: usize,
) -> f64 {
    let escape_per_variant = switch_rate / variants.max(1) as f64;
    immune_recognition * (-escape_per_variant * variants as f64).exp()
}

pub fn immune_evasion_molecular_mimicry(
    host_molecule_similarity: f64,
    immune_response_base: f64,
) -> f64 {
    immune_response_base * (1.0 - host_molecule_similarity)
}

pub fn immunosuppression_by_parasite(
    il10_induction: f64,
    treg_expansion: f64,
    effector_response: f64,
) -> f64 {
    effector_response / (1.0 + il10_induction + treg_expansion)
}

pub fn encapsulation_melanization(
    hemocyte_density: f64,
    parasite_surface_area: f64,
    phenoloxidase: f64,
) -> f64 {
    hemocyte_density * phenoloxidase / (1.0 + parasite_surface_area)
}

pub fn acquired_immunity_buildup(exposure_events: usize, max_immunity: f64, rate: f64) -> f64 {
    max_immunity * (1.0 - (-rate * exposure_events as f64).exp())
}

pub fn maternal_antibody_decay(initial_titer: f64, half_life_weeks: f64, age_weeks: f64) -> f64 {
    initial_titer * (-0.693 * age_weeks / half_life_weeks.max(1e-30)).exp()
}

pub fn concomitant_immunity(adult_worms: f64, larval_killing_rate: f64, new_larvae: f64) -> f64 {
    new_larvae * (-larval_killing_rate * adult_worms).exp()
}

pub fn eosinophil_response(parasite_burden: f64, il5_level: f64, eosinophil_base: f64) -> f64 {
    eosinophil_base * (1.0 + il5_level * parasite_burden)
}

pub fn granuloma_formation_rate(
    antigen_deposition: f64,
    macrophage_activation: f64,
    fibrosis_rate: f64,
) -> f64 {
    antigen_deposition * macrophage_activation * (1.0 + fibrosis_rate)
}

pub fn hygiene_hypothesis_index(
    parasite_exposure: f64,
    allergy_risk_base: f64,
    protection_factor: f64,
) -> f64 {
    allergy_risk_base / (1.0 + protection_factor * parasite_exposure)
}
