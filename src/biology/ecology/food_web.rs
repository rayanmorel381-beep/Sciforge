pub fn lotka_volterra_competition(
    n1: f64,
    n2: f64,
    r1: f64,
    r2: f64,
    k1: f64,
    k2: f64,
    alpha12: f64,
    alpha21: f64,
) -> (f64, f64) {
    let dn1 = r1 * n1 * (1.0 - (n1 + alpha12 * n2) / k1);
    let dn2 = r2 * n2 * (1.0 - (n2 + alpha21 * n1) / k2);
    (dn1, dn2)
}

pub fn lotka_volterra_predator_prey(
    prey: f64,
    predator: f64,
    r: f64,
    a: f64,
    b: f64,
    m: f64,
) -> (f64, f64) {
    let d_prey = r * prey - a * prey * predator;
    let d_pred = b * a * prey * predator - m * predator;
    (d_prey, d_pred)
}

pub fn rosenzweig_macarthur(
    prey: f64,
    predator: f64,
    r: f64,
    k: f64,
    a: f64,
    h: f64,
    e: f64,
    m: f64,
) -> (f64, f64) {
    let functional_response = a * prey / (1.0 + a * h * prey);
    let d_prey = r * prey * (1.0 - prey / k) - functional_response * predator;
    let d_pred = e * functional_response * predator - m * predator;
    (d_prey, d_pred)
}

pub fn type_ii_functional_response(prey_density: f64, attack_rate: f64, handling_time: f64) -> f64 {
    attack_rate * prey_density / (1.0 + attack_rate * handling_time * prey_density)
}

pub fn type_iii_functional_response(
    prey_density: f64,
    attack_rate: f64,
    handling_time: f64,
    exponent: f64,
) -> f64 {
    let n_exp = prey_density.powf(exponent);
    attack_rate * n_exp / (1.0 + attack_rate * handling_time * n_exp)
}

pub fn nutrient_cycling(
    nutrient: f64,
    producers: f64,
    decomposers: f64,
    uptake_rate: f64,
    mortality_rate: f64,
    decomposition_rate: f64,
) -> (f64, f64, f64) {
    let uptake = uptake_rate * nutrient * producers;
    let death = mortality_rate * producers;
    let decomp = decomposition_rate * decomposers * death;
    let dn = -uptake + decomp;
    let dp = uptake - death;
    let dd = death - decomp;
    (dn, dp, dd)
}

pub fn disturbance_regime(
    biomass: f64,
    disturbance_intensity: f64,
    return_interval: f64,
    time_since: f64,
) -> f64 {
    if time_since >= return_interval {
        biomass * (1.0 - disturbance_intensity)
    } else {
        biomass
    }
}

pub fn intermediate_disturbance_diversity(
    disturbance_frequency: f64,
    max_diversity: f64,
    optimal_frequency: f64,
) -> f64 {
    max_diversity
        * (-((disturbance_frequency - optimal_frequency) / optimal_frequency).powi(2)).exp()
}

pub fn metapopulation_levins(p: f64, colonization: f64, extinction: f64) -> f64 {
    colonization * p * (1.0 - p) - extinction * p
}

pub fn source_sink_dynamics(
    source_emigration: f64,
    sink_mortality: f64,
    sink_immigration: f64,
) -> f64 {
    sink_immigration + source_emigration - sink_mortality
}

pub fn food_web_connectance(links: usize, species: usize) -> f64 {
    if species < 2 {
        return 0.0;
    }
    links as f64 / (species * species) as f64
}

pub fn trophic_level(diet_trophic_levels: &[f64], diet_fractions: &[f64]) -> f64 {
    let n = diet_trophic_levels.len().min(diet_fractions.len());
    let sum: f64 = (0..n)
        .map(|i| diet_trophic_levels[i] * diet_fractions[i])
        .sum();
    1.0 + sum
}

pub fn lindeman_efficiency(energy_n_plus_1: f64, energy_n: f64) -> f64 {
    energy_n_plus_1 / energy_n.max(1e-30)
}
