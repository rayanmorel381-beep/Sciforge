pub fn telomere_shortening(initial_length: f64, loss_per_division: f64, divisions: f64) -> f64 {
    (initial_length - loss_per_division * divisions).max(0.0)
}

pub fn hayflick_limit(initial_length: f64, critical_length: f64, loss_per_division: f64) -> f64 {
    ((initial_length - critical_length) / loss_per_division).max(0.0)
}

pub fn telomerase_equilibrium(
    shortening_rate: f64,
    elongation_rate: f64,
    initial: f64,
    t: f64,
) -> f64 {
    let eq = elongation_rate / shortening_rate;
    eq + (initial - eq) * (-shortening_rate * t).exp()
}

pub fn oxidative_damage_accumulation(
    production_rate: f64,
    repair_rate: f64,
    t: f64,
    initial_damage: f64,
) -> f64 {
    let eq = production_rate / repair_rate;
    eq + (initial_damage - eq) * (-repair_rate * t).exp()
}

pub fn mitochondrial_damage(intact_fraction: f64, damage_rate: f64, dt: f64) -> f64 {
    (intact_fraction * (1.0 - damage_rate * dt)).max(0.0)
}

pub fn senescent_cell_fraction(
    division_rate: f64,
    senescence_prob: f64,
    clearance_rate: f64,
    t: f64,
) -> f64 {
    let eq = division_rate * senescence_prob / clearance_rate;
    eq * (1.0 - (-clearance_rate * t).exp())
}

pub fn caloric_restriction_lifespan(
    base_lifespan: f64,
    restriction_fraction: f64,
    effect_coefficient: f64,
) -> f64 {
    base_lifespan * (1.0 + effect_coefficient * restriction_fraction)
}

pub fn reliability_theory_survival(
    n_elements: usize,
    element_failure_rate: f64,
    redundancy: usize,
    t: f64,
) -> f64 {
    let single_element_survival = (-element_failure_rate * t).exp();
    let block_failure = (1.0 - single_element_survival).powi(redundancy as i32);
    (1.0 - block_failure).powi(n_elements as i32)
}

pub fn ros_steady_state(production_rate: f64, sod_activity: f64, catalase_activity: f64) -> f64 {
    production_rate / (sod_activity + catalase_activity).max(1e-30)
}

pub fn protein_aggregation(
    misfolded: f64,
    aggregation_rate: f64,
    chaperone_capacity: f64,
    dt: f64,
) -> f64 {
    let net_rate = aggregation_rate * misfolded * (misfolded - chaperone_capacity).max(0.0);
    misfolded + net_rate * dt
}

pub fn dna_repair_capacity(age: f64, base_capacity: f64, decline_rate: f64) -> f64 {
    base_capacity * (-decline_rate * age).exp()
}

pub fn somatic_mutation_accumulation(
    mutation_rate: f64,
    divisions: f64,
    repair_efficiency: f64,
) -> f64 {
    mutation_rate * divisions * (1.0 - repair_efficiency)
}

pub fn epigenetic_clock_horvath(cpg_values: &[f64], coefficients: &[f64], intercept: f64) -> f64 {
    let sum: f64 = cpg_values
        .iter()
        .zip(coefficients.iter())
        .map(|(&v, &c)| v * c)
        .sum();
    sum + intercept
}

pub fn nad_decline(initial_nad: f64, decline_rate: f64, age: f64) -> f64 {
    initial_nad * (-decline_rate * age).exp()
}

pub fn autophagy_flux(
    substrate: f64,
    autophagosome_formation: f64,
    lysosomal_activity: f64,
    age_factor: f64,
) -> f64 {
    autophagosome_formation * lysosomal_activity * age_factor * substrate / (substrate + 1.0)
}

pub fn stem_cell_exhaustion(
    initial_pool: f64,
    division_rate: f64,
    senescence_prob: f64,
    age: f64,
) -> f64 {
    initial_pool * (-division_rate * senescence_prob * age).exp()
}

pub fn inflammaging_cytokine(basal: f64, senescent_cells: f64, amplification: f64) -> f64 {
    basal + amplification * senescent_cells
}

pub fn crosslink_accumulation(rate: f64, turnover: f64, t: f64) -> f64 {
    rate / turnover * (1.0 - (-turnover * t).exp())
}

pub fn lipofuscin_accumulation(production_rate: f64, t: f64) -> f64 {
    production_rate * t
}

pub fn immune_senescence(
    naive_t_cells: f64,
    thymic_output_rate: f64,
    age: f64,
    proliferation_capacity: f64,
) -> f64 {
    naive_t_cells * (-thymic_output_rate * age).exp()
        + proliferation_capacity * (1.0 - (-0.01 * age).exp())
}
