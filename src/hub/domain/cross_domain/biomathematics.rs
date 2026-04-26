//! Biomathematics: logistic growth, Lotka-Volterra dynamics,
//! epidemiological models (SIR), and population genetics.

pub fn logistic_growth_rate(r: f64, carrying_capacity: f64, population: f64) -> f64 {
    r * population * (1.0 - population / carrying_capacity)
}

pub fn lotka_volterra_prey_rate(
    growth_rate: f64,
    predation_rate: f64,
    prey: f64,
    predator: f64,
) -> f64 {
    growth_rate * prey - predation_rate * prey * predator
}

pub fn lotka_volterra_predator_rate(
    conversion_rate: f64,
    death_rate: f64,
    prey: f64,
    predator: f64,
) -> f64 {
    conversion_rate * prey * predator - death_rate * predator
}

pub fn sir_infection_rate(beta: f64, susceptible: f64, infected: f64, total: f64) -> f64 {
    -beta * susceptible * infected / total
}

pub fn sir_recovery_rate(gamma: f64, infected: f64) -> f64 {
    gamma * infected
}

pub fn basic_reproduction_number(beta: f64, gamma: f64) -> f64 {
    beta / gamma
}

pub fn shannon_diversity_index(proportions: &[f64]) -> f64 {
    -proportions
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| p * p.ln())
        .sum::<f64>()
}

pub fn simpson_diversity_index(proportions: &[f64]) -> f64 {
    1.0 - proportions.iter().map(|&p| p * p).sum::<f64>()
}

pub fn molecular_clock_distance(substitution_rate: f64, time: f64) -> f64 {
    2.0 * substitution_rate * time
}

pub fn coalescent_expected_time(effective_population: f64, n_lineages: f64) -> f64 {
    4.0 * effective_population / (n_lineages * (n_lineages - 1.0))
}

pub fn wright_fisher_heterozygosity(h0: f64, effective_population: f64, generations: f64) -> f64 {
    h0 * (1.0 - 1.0 / (2.0 * effective_population)).powf(generations)
}
