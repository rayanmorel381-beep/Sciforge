fn lcg_next(state: u64) -> u64 {
    state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

pub fn genetic_drift_wright_fisher(
    p: f64,
    pop_size: usize,
    generations: usize,
    seed: u64,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(generations + 1);
    let mut freq = p;
    let mut rng_state = seed;
    result.push(freq);
    let n = 2 * pop_size;
    for _ in 0..generations {
        let mut count = 0u64;
        for _ in 0..n {
            rng_state = lcg_next(rng_state);
            let r = (rng_state as f64) / (u64::MAX as f64);
            if r < freq {
                count += 1;
            }
        }
        freq = count as f64 / n as f64;
        result.push(freq);
    }
    result
}

pub fn drift_effective_population_size(n_males: f64, n_females: f64) -> f64 {
    4.0 * n_males * n_females / (n_males + n_females)
}

pub fn effective_population_size_varying(sizes: &[f64]) -> f64 {
    let t = sizes.len() as f64;
    let harmonic_sum: f64 = sizes.iter().map(|&n| 1.0 / n).sum();
    t / harmonic_sum
}

pub fn heterozygosity_loss(ne: f64, generations: usize) -> f64 {
    (1.0 - 1.0 / (2.0 * ne)).powi(generations as i32)
}

pub fn mutation_drift_equilibrium(ne: f64, mu: f64) -> f64 {
    4.0 * ne * mu / (1.0 + 4.0 * ne * mu)
}

pub fn fixation_probability_neutral(ne: f64) -> f64 {
    1.0 / (2.0 * ne)
}

pub fn fixation_probability_selection(ne: f64, s: f64, p: f64) -> f64 {
    if s.abs() < 1e-15 {
        return p;
    }
    let num = 1.0 - (-2.0 * ne * s * p).exp();
    let den = 1.0 - (-2.0 * ne * s).exp();
    if den.abs() < 1e-30 {
        return p;
    }
    num / den
}

pub fn fixation_time_neutral(ne: f64) -> f64 {
    4.0 * ne
}

pub fn bottleneck_heterozygosity(h0: f64, n_bottleneck: f64, generations: usize) -> f64 {
    h0 * (1.0 - 1.0 / (2.0 * n_bottleneck)).powi(generations as i32)
}
