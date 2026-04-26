pub fn somatic_affinity_maturation(
    initial_kd: f64,
    rounds: usize,
    improvement_per_round: f64,
) -> f64 {
    initial_kd * (-improvement_per_round * rounds as f64).exp()
}

pub fn antibody_titer(dilution_factor: f64, endpoint_dilution: usize) -> f64 {
    dilution_factor.powi(endpoint_dilution as i32)
}

pub fn isotype_switch_probability(
    cytokine_signal: f64,
    switch_region_accessibility: f64,
    aid_activity: f64,
) -> f64 {
    let p = cytokine_signal * switch_region_accessibility * aid_activity;
    p.min(1.0)
}

pub fn somatic_hypermutation_rate(aid_expression: f64, base_rate: f64, hotspot_factor: f64) -> f64 {
    base_rate * aid_expression * hotspot_factor
}

pub fn neutralization_potency(ic50: f64, virus_titer: f64) -> f64 {
    virus_titer / ic50.max(1e-30)
}

pub fn opsonization_index(
    antibody_bound: f64,
    fc_receptor_density: f64,
    complement_coating: f64,
) -> f64 {
    antibody_bound * (fc_receptor_density + complement_coating)
}

pub fn adcc_killing_rate(
    antibody_density: f64,
    nk_cell_count: f64,
    target_count: f64,
    k_sat: f64,
) -> f64 {
    nk_cell_count * antibody_density * target_count / (k_sat + target_count)
}

pub fn complement_cascade_c3b(c3: f64, convertase_activity: f64, factor_h_inhibition: f64) -> f64 {
    convertase_activity * c3 / (1.0 + factor_h_inhibition)
}

pub fn multivalent_avidity(monovalent_kd: f64, valency: usize, cooperativity: f64) -> f64 {
    monovalent_kd / (valency as f64).powf(cooperativity)
}

pub fn elisa_concentration(od: f64, od_max: f64, ec50: f64, hill: f64) -> f64 {
    if od >= od_max {
        return f64::INFINITY;
    }
    let ratio = od / (od_max - od).max(1e-30);
    ec50 * ratio.powf(1.0 / hill)
}

pub fn b_cell_germinal_center_selection(affinity: f64, threshold: f64, t_cell_help: f64) -> bool {
    affinity * t_cell_help > threshold
}
