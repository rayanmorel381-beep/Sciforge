pub fn speciation_rate_bdi(lambda: f64, mu: f64, t: f64, n0: f64) -> f64 {
    n0 * ((lambda - mu) * t).exp()
}

pub fn allopatric_divergence(d0: f64, mu: f64, t: f64) -> f64 {
    d0 + 2.0 * mu * t
}

pub fn reproductive_isolation(genetic_distance: f64, k: f64, n: f64) -> f64 {
    let gn = genetic_distance.powf(n);
    gn / (k.powf(n) + gn)
}

pub fn reinforcement_strength(sympatry: f64, hybrid_fitness: f64) -> f64 {
    sympatry * (1.0 - hybrid_fitness)
}

pub fn yule_process_expected_species(lambda: f64, t: f64) -> f64 {
    (lambda * t).exp()
}

pub fn birth_death_survival(lambda: f64, mu: f64, t: f64) -> f64 {
    if (lambda - mu).abs() < 1e-12 {
        return 1.0 / (1.0 + lambda * t);
    }
    let r = lambda - mu;
    let ert = (r * t).exp();
    r / (lambda * ert - mu)
}

pub fn character_displacement(z1: f64, z2: f64, alpha: f64, sigma: f64) -> (f64, f64) {
    let overlap = (-(z1 - z2).powi(2) / (2.0 * sigma * sigma)).exp();
    let force = alpha * overlap;
    let dz1 = -force * (z1 - z2);
    let dz2 = force * (z1 - z2);
    (dz1, dz2)
}

pub fn ecological_speciation_fitness(
    trait_val: f64,
    optimum1: f64,
    optimum2: f64,
    sigma: f64,
) -> (f64, f64) {
    let w1 = (-(trait_val - optimum1).powi(2) / (2.0 * sigma * sigma)).exp();
    let w2 = (-(trait_val - optimum2).powi(2) / (2.0 * sigma * sigma)).exp();
    (w1, w2)
}
