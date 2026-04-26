pub fn monte_carlo_integrate(f: fn(f64) -> f64, a: f64, b: f64, n: usize, seed: u64) -> f64 {
    let mut rng = McRng::new(seed);
    let mut sum = 0.0;
    for _ in 0..n {
        let x = a + rng.next_f64() * (b - a);
        sum += f(x);
    }
    (b - a) * sum / n as f64
}

pub fn monte_carlo_pi(n: usize, seed: u64) -> f64 {
    let mut rng = McRng::new(seed);
    let mut inside = 0u64;
    for _ in 0..n {
        let x = rng.next_f64();
        let y = rng.next_f64();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    4.0 * inside as f64 / n as f64
}

pub fn importance_sampling(
    f: fn(f64) -> f64,
    proposal_sample: fn(&mut McRng) -> f64,
    proposal_pdf: fn(f64) -> f64,
    target_pdf: fn(f64) -> f64,
    n: usize,
    seed: u64,
) -> f64 {
    let mut rng = McRng::new(seed);
    let mut sum = 0.0;
    for _ in 0..n {
        let x = proposal_sample(&mut rng);
        let w = target_pdf(x) / proposal_pdf(x);
        sum += f(x) * w;
    }
    sum / n as f64
}

pub fn metropolis_hastings(
    target_log_pdf: fn(f64) -> f64,
    proposal_step: f64,
    x0: f64,
    n_samples: usize,
    burn_in: usize,
    seed: u64,
) -> Vec<f64> {
    let mut rng = McRng::new(seed);
    let mut x = x0;
    let mut samples = Vec::with_capacity(n_samples);
    for i in 0..(n_samples + burn_in) {
        let candidate = x + proposal_step * (rng.next_f64() - 0.5) * 2.0;
        let log_alpha = target_log_pdf(candidate) - target_log_pdf(x);
        if log_alpha >= 0.0 || rng.next_f64() < log_alpha.exp() {
            x = candidate;
        }
        if i >= burn_in {
            samples.push(x);
        }
    }
    samples
}

pub fn rejection_sampling(
    target_pdf: fn(f64) -> f64,
    proposal_sample: fn(&mut McRng) -> f64,
    proposal_pdf: fn(f64) -> f64,
    m: f64,
    n_samples: usize,
    seed: u64,
) -> Vec<f64> {
    let mut rng = McRng::new(seed);
    let mut samples = Vec::with_capacity(n_samples);
    while samples.len() < n_samples {
        let x = proposal_sample(&mut rng);
        let u = rng.next_f64();
        if u < target_pdf(x) / (m * proposal_pdf(x)) {
            samples.push(x);
        }
    }
    samples
}

pub fn bootstrap_mean(data: &[f64], n_bootstrap: usize, seed: u64) -> (f64, f64) {
    let mut rng = McRng::new(seed);
    let n = data.len();
    let mut means = Vec::with_capacity(n_bootstrap);
    for _ in 0..n_bootstrap {
        let mut sum = 0.0;
        for _ in 0..n {
            let idx = rng.next_usize(n);
            sum += data[idx];
        }
        means.push(sum / n as f64);
    }
    let mean: f64 = means.iter().sum::<f64>() / n_bootstrap as f64;
    let var: f64 = means.iter().map(|m| (m - mean).powi(2)).sum::<f64>() / (n_bootstrap - 1) as f64;
    (mean, var.sqrt())
}

pub struct McRng {
    state: u64,
}
impl McRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn next_usize(&mut self, max: usize) -> usize {
        (self.next_f64() * max as f64) as usize % max
    }
}

pub fn monte_carlo_integrate_nd(
    f: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    n: usize,
    seed: u64,
) -> f64 {
    let mut rng = McRng::new(seed);
    let dims = bounds.len();
    let volume: f64 = bounds.iter().map(|(a, b)| b - a).product();
    let mut sum = 0.0;
    let mut point = vec![0.0; dims];
    for _ in 0..n {
        for d in 0..dims {
            point[d] = bounds[d].0 + rng.next_f64() * (bounds[d].1 - bounds[d].0);
        }
        sum += f(&point);
    }
    volume * sum / n as f64
}

pub fn monte_carlo_variance(f: fn(f64) -> f64, a: f64, b: f64, n: usize, seed: u64) -> f64 {
    let mut rng = McRng::new(seed);
    let mut sum = 0.0;
    let mut sum2 = 0.0;
    for _ in 0..n {
        let x = a + rng.next_f64() * (b - a);
        let val = f(x);
        sum += val;
        sum2 += val * val;
    }
    let mean = sum / n as f64;
    (b - a) * (b - a) * (sum2 / n as f64 - mean * mean)
}

pub fn antithetic_variates(f: fn(f64) -> f64, a: f64, b: f64, n: usize, seed: u64) -> f64 {
    let mut rng = McRng::new(seed);
    let mut sum = 0.0;
    for _ in 0..n {
        let u = rng.next_f64();
        let x1 = a + u * (b - a);
        let x2 = a + (1.0 - u) * (b - a);
        sum += (f(x1) + f(x2)) / 2.0;
    }
    (b - a) * sum / n as f64
}

pub fn control_variates(
    f: fn(f64) -> f64,
    g: fn(f64) -> f64,
    expected_g: f64,
    a: f64,
    b: f64,
    n: usize,
    seed: u64,
) -> f64 {
    let mut rng = McRng::new(seed);
    let mut f_vals = Vec::with_capacity(n);
    let mut g_vals = Vec::with_capacity(n);
    for _ in 0..n {
        let x = a + rng.next_f64() * (b - a);
        f_vals.push(f(x));
        g_vals.push(g(x));
    }
    let f_mean: f64 = f_vals.iter().sum::<f64>() / n as f64;
    let g_mean: f64 = g_vals.iter().sum::<f64>() / n as f64;
    let mut cov = 0.0;
    let mut var_g = 0.0;
    for i in 0..n {
        cov += (f_vals[i] - f_mean) * (g_vals[i] - g_mean);
        var_g += (g_vals[i] - g_mean).powi(2);
    }
    let c_star = if var_g > 1e-30 { -cov / var_g } else { 0.0 };
    (b - a) * (f_mean + c_star * (g_mean - expected_g / (b - a)))
}

pub fn quasi_monte_carlo_integrate(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let mut sum = 0.0;
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let alpha = 1.0 / phi;
    for i in 0..n {
        let u = ((0.5 + i as f64 * alpha) % 1.0) * (b - a) + a;
        sum += f(u);
    }
    (b - a) * sum / n as f64
}

pub fn error_propagation(partials: &[f64], uncertainties: &[f64]) -> f64 {
    partials
        .iter()
        .zip(uncertainties.iter())
        .map(|(&df, &sigma)| (df * sigma).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn monte_carlo_uncertainty(
    f: fn(&[f64]) -> f64,
    means: &[f64],
    sigmas: &[f64],
    n: usize,
    seed: u64,
) -> f64 {
    let mut rng = seed;
    let mut results = Vec::with_capacity(n);
    for _ in 0..n {
        let sample: Vec<f64> = means
            .iter()
            .zip(sigmas.iter())
            .map(|(&mu, &sigma)| {
                rng = rng
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let u1 = (rng >> 33) as f64 / u32::MAX as f64 + 1e-15;
                rng = rng
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let u2 = (rng >> 33) as f64 / u32::MAX as f64;
                let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                mu + sigma * z
            })
            .collect();
        results.push(f(&sample));
    }
    let mean = results.iter().sum::<f64>() / n as f64;
    (results.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt()
}

pub fn gibbs_sampler_2d(
    sample_x_given_y: fn(f64, &mut McRng) -> f64,
    sample_y_given_x: fn(f64, &mut McRng) -> f64,
    n_samples: usize,
    burn_in: usize,
    seed: u64,
) -> Vec<(f64, f64)> {
    let mut rng = McRng::new(seed);
    let mut x = 0.0;
    let mut y = sample_y_given_x(x, &mut rng);
    let mut samples = Vec::with_capacity(n_samples);
    for i in 0..(n_samples + burn_in) {
        x = sample_x_given_y(y, &mut rng);
        y = sample_y_given_x(x, &mut rng);
        if i >= burn_in {
            samples.push((x, y));
        }
    }
    samples
}

pub fn slice_sampling(
    log_pdf: fn(f64) -> f64,
    x0: f64,
    w: f64,
    n_samples: usize,
    burn_in: usize,
    seed: u64,
) -> Vec<f64> {
    let mut rng = McRng::new(seed);
    let mut x = x0;
    let mut samples = Vec::with_capacity(n_samples);
    for i in 0..(n_samples + burn_in) {
        let log_y = log_pdf(x) + rng.next_f64().ln();
        let mut left = x - w * rng.next_f64();
        let mut right = left + w;
        while log_pdf(left) > log_y {
            left -= w;
        }
        while log_pdf(right) > log_y {
            right += w;
        }
        loop {
            let x_new = left + rng.next_f64() * (right - left);
            if log_pdf(x_new) > log_y {
                x = x_new;
                break;
            }
            if x_new < x {
                left = x_new;
            } else {
                right = x_new;
            }
        }
        if i >= burn_in {
            samples.push(x);
        }
    }
    samples
}

pub fn permutation_test(group_a: &[f64], group_b: &[f64], n_permutations: usize, seed: u64) -> f64 {
    let mut rng = McRng::new(seed);
    let observed = group_a.iter().sum::<f64>() / group_a.len() as f64
        - group_b.iter().sum::<f64>() / group_b.len() as f64;
    let observed_abs = observed.abs();
    let mut combined: Vec<f64> = group_a.iter().chain(group_b.iter()).cloned().collect();
    let na = group_a.len();
    let mut count = 0;
    for _ in 0..n_permutations {
        for i in (1..combined.len()).rev() {
            let j = rng.next_usize(i + 1);
            combined.swap(i, j);
        }
        let mean_a: f64 = combined[..na].iter().sum::<f64>() / na as f64;
        let mean_b: f64 = combined[na..].iter().sum::<f64>() / (combined.len() - na) as f64;
        if (mean_a - mean_b).abs() >= observed_abs {
            count += 1;
        }
    }
    count as f64 / n_permutations as f64
}
