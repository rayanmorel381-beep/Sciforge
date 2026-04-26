pub fn molecular_clock_rate(substitutions: f64, divergence_time: f64) -> f64 {
    substitutions / (2.0 * divergence_time).max(1e-30)
}

pub fn strict_clock_branch_length(rate: f64, time: f64) -> f64 {
    rate * time
}

pub fn relaxed_clock_lognormal(mean_rate: f64, sigma: f64, branch_deviation: f64) -> f64 {
    mean_rate * (sigma * branch_deviation - sigma * sigma / 2.0).exp()
}

pub fn divergence_time_from_distance(genetic_distance: f64, substitution_rate: f64) -> f64 {
    genetic_distance / (2.0 * substitution_rate).max(1e-30)
}

pub fn jc_distance(p_diff: f64) -> f64 {
    -0.75 * (1.0 - 4.0 * p_diff / 3.0).max(1e-30).ln()
}

pub fn kimura_2p_distance(transitions: f64, transversions: f64, length: f64) -> f64 {
    let p = transitions / length.max(1e-30);
    let q = transversions / length.max(1e-30);
    -0.5 * (1.0 - 2.0 * p - q).max(1e-30).ln() - 0.25 * (1.0 - 2.0 * q).max(1e-30).ln()
}

pub fn bayesian_clock_calibration(
    prior_age: f64,
    prior_sd: f64,
    likelihood_age: f64,
    likelihood_sd: f64,
) -> (f64, f64) {
    let posterior_var = 1.0 / (1.0 / (prior_sd * prior_sd) + 1.0 / (likelihood_sd * likelihood_sd));
    let posterior_mean = posterior_var
        * (prior_age / (prior_sd * prior_sd) + likelihood_age / (likelihood_sd * likelihood_sd));
    (posterior_mean, posterior_var.sqrt())
}

pub fn rate_heterogeneity_gamma(alpha: f64, n_categories: usize) -> Vec<f64> {
    let mut rates = Vec::with_capacity(n_categories);
    for i in 0..n_categories {
        let p = (2 * i + 1) as f64 / (2 * n_categories) as f64;
        let rate = alpha * (1.0 + (p - 0.5) * 2.0 / alpha.sqrt());
        rates.push(rate.max(0.01));
    }
    rates
}

pub fn local_clock_assignment(branch_rates: &[f64], threshold: f64) -> Vec<usize> {
    if branch_rates.is_empty() {
        return vec![];
    }
    let mean: f64 = branch_rates.iter().sum::<f64>() / branch_rates.len() as f64;
    branch_rates
        .iter()
        .map(|&r| {
            if r < mean * (1.0 - threshold) {
                0
            } else if r > mean * (1.0 + threshold) {
                2
            } else {
                1
            }
        })
        .collect()
}
