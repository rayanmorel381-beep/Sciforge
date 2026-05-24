pub fn self_renewal_probability(symmetric_rate: f64, total_division_rate: f64) -> f64 {
    symmetric_rate / total_division_rate
}

pub fn stem_cell_pool_dynamics(s: f64, r: f64, d: f64, p: f64, dt: f64) -> f64 {
    let ds = (2.0 * p - 1.0) * r * s - d * s;
    (s + ds * dt).max(0.0)
}

pub fn asymmetric_division_output(
    stem_cells: f64,
    division_rate: f64,
    asymmetric_fraction: f64,
) -> f64 {
    stem_cells * division_rate * asymmetric_fraction
}

pub fn lineage_commitment(signal_strength: f64, threshold: f64, hill_n: f64) -> f64 {
    signal_strength.powf(hill_n) / (threshold.powf(hill_n) + signal_strength.powf(hill_n))
}

pub fn niche_occupancy(stem_cells: f64, niche_capacity: f64) -> f64 {
    (stem_cells / niche_capacity).min(1.0)
}

pub fn niche_competition(
    resident: f64,
    challenger: f64,
    fitness_resident: f64,
    fitness_challenger: f64,
) -> f64 {
    fitness_challenger / (fitness_resident + fitness_challenger) * (resident + challenger)
}

pub fn dedifferentiation_rate(injury_signal: f64, plasticity: f64, baseline: f64) -> f64 {
    baseline + plasticity * injury_signal
}

pub fn stem_cell_aging(initial_pool: f64, depletion_rate: f64, age: f64) -> f64 {
    initial_pool * (-depletion_rate * age).exp()
}

pub fn transit_amplifying_generations(
    progenitor: f64,
    divisions: u32,
    survival_per_div: f64,
) -> f64 {
    progenitor * (2.0 * survival_per_div).powi(divisions as i32)
}

pub fn quiescence_exit_rate(growth_factor: f64, threshold: f64, max_rate: f64) -> f64 {
    max_rate * growth_factor.powi(2) / (threshold.powi(2) + growth_factor.powi(2))
}

pub fn clonal_dominance(fitness: &[f64]) -> Vec<f64> {
    let total: f64 = fitness.iter().sum();
    if total <= 0.0 {
        return vec![0.0; fitness.len()];
    }
    fitness.iter().map(|&f| f / total).collect()
}

pub fn neutral_drift_clone_survival(initial_clones: f64, time: f64, replacement_rate: f64) -> f64 {
    initial_clones / (1.0 + replacement_rate * time)
}

pub fn hematopoietic_hierarchy_output(
    hsc: f64,
    mpp_rate: f64,
    clp_rate: f64,
    cmp_rate: f64,
) -> (f64, f64) {
    let mpp = hsc * mpp_rate;
    let lymphoid = mpp * clp_rate;
    let myeloid = mpp * cmp_rate;
    (lymphoid, myeloid)
}

pub fn telomere_shortening_per_division(
    initial_length: f64,
    loss_per_division: f64,
    divisions: f64,
) -> f64 {
    (initial_length - loss_per_division * divisions).max(0.0)
}

pub fn hayflick_limit_remaining(
    telomere_length: f64,
    critical_length: f64,
    loss_per_division: f64,
) -> f64 {
    ((telomere_length - critical_length) / loss_per_division).max(0.0)
}

pub fn symmetric_commitment_probability(niche_signal: f64, k_niche: f64) -> f64 {
    1.0 - niche_signal / (niche_signal + k_niche)
}
