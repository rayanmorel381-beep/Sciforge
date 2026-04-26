pub fn reprogramming_efficiency(
    oct4: f64,
    sox2: f64,
    klf4: f64,
    myc: f64,
    epigenetic_barrier: f64,
) -> f64 {
    let factor_product = oct4 * sox2 * klf4 * myc;
    factor_product / (factor_product + epigenetic_barrier)
}

pub fn ipsc_colony_formation(
    seeded_cells: f64,
    reprogramming_efficiency: f64,
    survival_fraction: f64,
) -> f64 {
    seeded_cells * reprogramming_efficiency * survival_fraction
}

pub fn differentiation_cascade(progenitor: f64, rates: &[f64]) -> Vec<f64> {
    let mut populations = Vec::with_capacity(rates.len() + 1);
    populations.push(progenitor);
    let mut current = progenitor;
    for &rate in rates {
        current *= rate;
        populations.push(current);
    }
    populations
}

pub fn waddington_potential(state: f64, landscape: impl Fn(f64) -> f64, noise: f64) -> f64 {
    let gradient = (landscape(state + 0.001) - landscape(state - 0.001)) / 0.002;
    -gradient + noise
}

pub fn organoid_growth(cells: f64, growth_rate: f64, carrying_capacity: f64, dt: f64) -> f64 {
    let growth = growth_rate * cells * (1.0 - cells / carrying_capacity);
    (cells + growth * dt).max(0.0)
}

pub fn yamanaka_factor_dynamics(
    oct4: f64,
    sox2: f64,
    klf4: f64,
    myc: f64,
    dt: f64,
    degradation: f64,
) -> (f64, f64, f64, f64) {
    let os_synergy = oct4 * sox2 * 0.5;
    let d_oct4 = os_synergy - degradation * oct4;
    let d_sox2 = os_synergy - degradation * sox2;
    let d_klf4 = 0.1 * oct4 - degradation * klf4;
    let d_myc = 0.2 - degradation * myc;
    (
        (oct4 + d_oct4 * dt).max(0.0),
        (sox2 + d_sox2 * dt).max(0.0),
        (klf4 + d_klf4 * dt).max(0.0),
        (myc + d_myc * dt).max(0.0),
    )
}

pub fn stochastic_reprogramming_events(cells: usize, probability_per_cell: f64) -> f64 {
    1.0 - (1.0 - probability_per_cell).powi(cells as i32)
}

pub fn partial_reprogramming_state(
    methylation_age: f64,
    cycles: usize,
    reset_per_cycle: f64,
) -> f64 {
    let mut age = methylation_age;
    for _ in 0..cycles {
        age *= 1.0 - reset_per_cycle;
        age = age.max(0.0);
    }
    age
}

pub fn direct_lineage_conversion(
    efficiency_base: f64,
    tf_combination: &[f64],
    synergy: f64,
) -> f64 {
    let tf_product: f64 = tf_combination.iter().product();
    let tf_sum: f64 = tf_combination.iter().sum();
    efficiency_base * tf_product.powf(synergy) / (1.0 + tf_sum)
}

pub fn asymmetric_division_ratio(
    stem_cells: f64,
    symmetric_prob: f64,
    differentiation_rate: f64,
) -> (f64, f64) {
    let symmetric = stem_cells * symmetric_prob;
    let asymmetric = stem_cells * (1.0 - symmetric_prob);
    let new_stem = symmetric * 2.0 + asymmetric;
    let differentiated = asymmetric * differentiation_rate;
    (new_stem, differentiated)
}

pub fn epigenetic_barrier_height(
    methylation_level: f64,
    histone_marks: f64,
    chromatin_accessibility: f64,
) -> f64 {
    methylation_level * histone_marks / chromatin_accessibility.max(1e-30)
}

pub fn crispr_activation_efficiency(
    guide_specificity: f64,
    activator_strength: f64,
    chromatin_state: f64,
) -> f64 {
    guide_specificity * activator_strength * chromatin_state
}

pub fn embryoid_body_formation(
    single_cells: f64,
    aggregation_rate: f64,
    min_cells_per_eb: f64,
) -> f64 {
    (single_cells * aggregation_rate / min_cells_per_eb).floor()
}

pub fn directed_differentiation_yield(
    input_cells: f64,
    protocol_efficiency: f64,
    purity: f64,
) -> f64 {
    input_cells * protocol_efficiency * purity
}

pub fn maturation_index(marker_expression: &[f64], weights: &[f64]) -> f64 {
    let weighted_sum: f64 = marker_expression
        .iter()
        .zip(weights.iter())
        .map(|(&e, &w)| e * w)
        .sum();
    let weight_total: f64 = weights.iter().sum();
    weighted_sum / weight_total.max(1e-30)
}
