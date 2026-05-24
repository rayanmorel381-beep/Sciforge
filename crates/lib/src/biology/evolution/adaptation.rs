pub fn adaptation_rate(
    selection_coefficient: f64,
    mutation_rate: f64,
    population_size: f64,
) -> f64 {
    2.0 * population_size * selection_coefficient * mutation_rate
}

pub fn fisher_geometric_adaptation(phenotype: &[f64], optimum: &[f64]) -> f64 {
    let mut dist_sq = 0.0;
    let n = phenotype.len().min(optimum.len());
    for i in 0..n {
        dist_sq += (phenotype[i] - optimum[i]).powi(2);
    }
    (-dist_sq / 2.0).exp()
}

pub fn adaptive_walk_progress(
    current_fitness: f64,
    max_fitness: f64,
    beneficial_rate: f64,
    step: usize,
) -> f64 {
    max_fitness - (max_fitness - current_fitness) * (-beneficial_rate * step as f64).exp()
}

pub fn beneficial_mutation_fixation_probability(s: f64, ne: f64) -> f64 {
    if s.abs() < 1e-15 {
        return 1.0 / (2.0 * ne);
    }
    (1.0 - (-2.0 * s).exp()) / (1.0 - (-4.0 * ne * s).exp())
}

pub fn phenotypic_plasticity(
    genotype_value: f64,
    environment: f64,
    reaction_norm_slope: f64,
) -> f64 {
    genotype_value + reaction_norm_slope * environment
}

pub fn baldwin_effect(learning_rate: f64, genetic_assimilation: f64, generations: usize) -> f64 {
    1.0 - (-genetic_assimilation * learning_rate * generations as f64).exp()
}

pub fn red_queen_coevolution(
    host_fitness: f64,
    parasite_fitness: f64,
    host_adapt_rate: f64,
    parasite_adapt_rate: f64,
    dt: f64,
) -> (f64, f64) {
    let dh = host_adapt_rate * (1.0 - parasite_fitness) * dt;
    let dp = parasite_adapt_rate * host_fitness * dt;
    (host_fitness + dh, parasite_fitness + dp)
}

pub fn environmental_gradient_selection(
    position: f64,
    optimum_slope: f64,
    selection_width: f64,
    phenotype: f64,
) -> f64 {
    let optimum = optimum_slope * position;
    (-(phenotype - optimum).powi(2) / (2.0 * selection_width * selection_width)).exp()
}

pub fn frequency_dependent_selection(
    frequency: f64,
    baseline_fitness: f64,
    fd_coefficient: f64,
) -> f64 {
    baseline_fitness + fd_coefficient * (1.0 - 2.0 * frequency)
}

pub fn adaptive_radiation_rate(
    niche_count: usize,
    occupied: usize,
    diversification_rate: f64,
) -> f64 {
    diversification_rate * (niche_count as f64 - occupied as f64) / niche_count.max(1) as f64
}
