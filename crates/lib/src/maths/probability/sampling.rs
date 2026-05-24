pub fn latin_hypercube(n_samples: usize, n_dims: usize, seed: u64) -> Vec<Vec<f64>> {
    let mut rng = SampRng::new(seed);
    let mut result = vec![vec![0.0; n_dims]; n_samples];
    for d in 0..n_dims {
        let mut perm: Vec<usize> = (0..n_samples).collect();
        for i in (1..n_samples).rev() {
            let j = rng.next_usize(i + 1);
            perm.swap(i, j);
        }
        for (ri, &pi) in result.iter_mut().zip(&perm) {
            ri[d] = (pi as f64 + rng.next_f64()) / n_samples as f64;
        }
    }
    result
}

pub fn stratified_sampling(n_strata: usize, seed: u64) -> Vec<f64> {
    let mut rng = SampRng::new(seed);
    (0..n_strata)
        .map(|i| (i as f64 + rng.next_f64()) / n_strata as f64)
        .collect()
}

pub fn sobol_sequence_1d(n: usize) -> Vec<f64> {
    let mut result = vec![0.0; n];
    for i in 1..n {
        let mut c = 0u32;
        let mut v = i;
        while v & 1 == 0 {
            c += 1;
            v >>= 1;
        }
        let direction = 1u64 << (63 - c);
        let prev_bits = (result[i - 1] * (1u64 << 63) as f64) as u64;
        result[i] = (prev_bits ^ direction) as f64 / (1u64 << 63) as f64;
    }
    result
}

pub fn halton_sequence(n: usize, base: u64) -> Vec<f64> {
    (0..n).map(|i| halton_element(i as u64, base)).collect()
}

fn halton_element(mut index: u64, base: u64) -> f64 {
    let mut result = 0.0;
    let mut f = 1.0 / base as f64;
    index += 1;
    while index > 0 {
        result += f * (index % base) as f64;
        index /= base;
        f /= base as f64;
    }
    result
}

pub fn inverse_transform_exponential(u: f64, lambda: f64) -> f64 {
    -(1.0 - u).ln() / lambda
}

pub fn box_muller(u1: f64, u2: f64) -> (f64, f64) {
    let r = (-2.0 * u1.ln()).sqrt();
    let theta = 2.0 * std::f64::consts::PI * u2;
    (r * theta.cos(), r * theta.sin())
}

pub fn reservoir_sampling(stream: &[f64], k: usize, seed: u64) -> Vec<f64> {
    let mut rng = SampRng::new(seed);
    let mut reservoir = stream[..k.min(stream.len())].to_vec();
    for (i, &si) in stream.iter().enumerate().skip(k) {
        let j = rng.next_usize(i + 1);
        if j < k {
            reservoir[j] = si;
        }
    }
    reservoir
}

struct SampRng {
    state: u64,
}
impl SampRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn next_usize(&mut self, max: usize) -> usize {
        (self.next_f64() * max as f64) as usize % max
    }
}

pub fn alias_table_build(probs: &[f64]) -> (Vec<f64>, Vec<usize>) {
    let n = probs.len();
    let avg = 1.0 / n as f64;
    let mut prob = vec![0.0; n];
    let mut alias = vec![0; n];
    let mut small = Vec::new();
    let mut large = Vec::new();
    let mut scaled: Vec<f64> = probs.iter().map(|p| p * n as f64).collect();
    for (i, &si) in scaled.iter().enumerate() {
        if si < 1.0 {
            small.push(i);
        } else {
            large.push(i);
        }
    }
    while let (Some(s), Some(l)) = (small.pop(), large.pop()) {
        prob[s] = scaled[s];
        alias[s] = l;
        scaled[l] = scaled[l] + scaled[s] - 1.0;
        if scaled[l] < 1.0 {
            small.push(l);
        } else {
            large.push(l);
        }
    }
    for &l in &large {
        prob[l] = 1.0;
    }
    for &s in &small {
        prob[s] = 1.0;
    }
    let _ = avg;
    (prob, alias)
}

pub fn alias_sample(prob: &[f64], alias: &[usize], seed: u64) -> usize {
    let mut rng = SampRng::new(seed);
    let n = prob.len();
    let i = rng.next_usize(n);
    if rng.next_f64() < prob[i] {
        i
    } else {
        alias[i]
    }
}

pub fn systematic_sampling(n_samples: usize, seed: u64) -> Vec<f64> {
    let mut rng = SampRng::new(seed);
    let u0 = rng.next_f64() / n_samples as f64;
    (0..n_samples)
        .map(|i| u0 + i as f64 / n_samples as f64)
        .collect()
}

pub fn importance_resampling(weights: &[f64], n_samples: usize, seed: u64) -> Vec<usize> {
    let mut rng = SampRng::new(seed);
    let total: f64 = weights.iter().sum();
    let normalized: Vec<f64> = weights.iter().map(|w| w / total).collect();
    let mut cumul = vec![0.0; normalized.len()];
    cumul[0] = normalized[0];
    for i in 1..normalized.len() {
        cumul[i] = cumul[i - 1] + normalized[i];
    }
    let mut indices = Vec::with_capacity(n_samples);
    for _ in 0..n_samples {
        let u = rng.next_f64();
        let idx = cumul.partition_point(|&c| c < u);
        indices.push(idx.min(weights.len() - 1));
    }
    indices
}

pub fn van_der_corput(n: usize, base: u64) -> Vec<f64> {
    (0..n)
        .map(|i| {
            let mut result = 0.0;
            let mut f = 1.0 / base as f64;
            let mut idx = (i + 1) as u64;
            while idx > 0 {
                result += f * (idx % base) as f64;
                idx /= base;
                f /= base as f64;
            }
            result
        })
        .collect()
}

pub fn hammersley_sequence(n: usize, base: u64) -> Vec<(f64, f64)> {
    let vdc = van_der_corput(n, base);
    (0..n).map(|i| (i as f64 / n as f64, vdc[i])).collect()
}

pub fn weighted_sampling(items: &[f64], weights: &[f64], n_samples: usize, seed: u64) -> Vec<f64> {
    let mut rng = SampRng::new(seed);
    let total: f64 = weights.iter().sum();
    let mut cumul = Vec::with_capacity(weights.len());
    let mut s = 0.0;
    for w in weights {
        s += w / total;
        cumul.push(s);
    }
    let mut samples = Vec::with_capacity(n_samples);
    for _ in 0..n_samples {
        let u = rng.next_f64();
        let idx = cumul.partition_point(|&c| c < u).min(items.len() - 1);
        samples.push(items[idx]);
    }
    samples
}

pub fn poisson_disk_sampling_1d(
    domain_min: f64,
    domain_max: f64,
    min_dist: f64,
    seed: u64,
) -> Vec<f64> {
    let mut rng = SampRng::new(seed);
    let mut points = Vec::new();
    let mut candidate = domain_min + rng.next_f64() * (domain_max - domain_min);
    points.push(candidate);
    let max_attempts = 30;
    let mut active = vec![0usize];
    while !active.is_empty() {
        let idx = rng.next_usize(active.len());
        let center = points[active[idx]];
        let mut found = false;
        for _ in 0..max_attempts {
            let offset = min_dist + rng.next_f64() * min_dist;
            candidate = if rng.next_f64() < 0.5 {
                center + offset
            } else {
                center - offset
            };
            if candidate < domain_min || candidate > domain_max {
                continue;
            }
            let ok = points.iter().all(|&p| (p - candidate).abs() >= min_dist);
            if ok {
                active.push(points.len());
                points.push(candidate);
                found = true;
                break;
            }
        }
        if !found {
            active.swap_remove(idx);
        }
    }
    points
}
