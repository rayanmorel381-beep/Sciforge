pub fn bayes_update(prior: &[f64], likelihood: &[f64]) -> Vec<f64> {
    let unnorm: Vec<f64> = prior
        .iter()
        .zip(likelihood.iter())
        .map(|(p, l)| p * l)
        .collect();
    let total: f64 = unnorm.iter().sum();
    unnorm.iter().map(|x| x / total).collect()
}

pub fn log_posterior(log_prior: f64, log_likelihood: f64) -> f64 {
    log_prior + log_likelihood
}

pub fn maximum_a_posteriori(log_priors: &[f64], log_likelihoods: &[f64]) -> usize {
    log_priors
        .iter()
        .zip(log_likelihoods.iter())
        .enumerate()
        .map(|(i, (lp, ll))| (i, lp + ll))
        .fold((0, f64::NEG_INFINITY), |(bi, bv), (i, v)| {
            if v > bv { (i, v) } else { (bi, bv) }
        })
        .0
}

pub fn credible_interval(samples: &[f64], alpha: f64) -> (f64, f64) {
    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    let lo = (n as f64 * alpha / 2.0) as usize;
    let hi = (n as f64 * (1.0 - alpha / 2.0)) as usize;
    (sorted[lo.min(n - 1)], sorted[hi.min(n - 1)])
}

pub fn metropolis_hastings(
    log_target: fn(f64) -> f64,
    init: f64,
    n: usize,
    step: f64,
    seed: u64,
) -> Vec<f64> {
    let mut rng = seed;
    let mut samples = Vec::with_capacity(n);
    let mut current = init;
    let mut log_current = log_target(current);
    for _ in 0..n {
        rng = rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let u = (rng >> 33) as f64 / u32::MAX as f64;
        let proposal = current + step * (2.0 * u - 1.0);
        let log_proposal = log_target(proposal);
        rng = rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let accept_u = (rng >> 33) as f64 / u32::MAX as f64;
        if accept_u.ln() < log_proposal - log_current {
            current = proposal;
            log_current = log_proposal;
        }
        samples.push(current);
    }
    samples
}

pub fn normal_log_likelihood(data: &[f64], mu: f64, sigma: f64) -> f64 {
    let n = data.len() as f64;
    let s2 = sigma * sigma;
    -n / 2.0 * (2.0 * std::f64::consts::PI * s2).ln()
        - data.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (2.0 * s2)
}

pub fn posterior_mean(samples: &[f64]) -> f64 {
    samples.iter().sum::<f64>() / samples.len() as f64
}

pub fn posterior_variance(samples: &[f64]) -> f64 {
    let mean = posterior_mean(samples);
    samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (samples.len() - 1) as f64
}
