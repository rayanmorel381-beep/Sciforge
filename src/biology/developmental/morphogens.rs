pub fn morphogen_gradient_steady(source: f64, decay: f64, diffusion: f64, x: f64) -> f64 {
    let lambda = (diffusion / decay).sqrt();
    source * (-(x.abs()) / lambda).exp()
}

pub fn morphogen_diffusion_1d(
    conc: &mut [f64],
    d: f64,
    decay: f64,
    source_idx: usize,
    source_rate: f64,
    dx: f64,
    dt: f64,
    steps: usize,
) {
    let n = conc.len();
    for _ in 0..steps {
        let old = conc.to_vec();
        for i in 1..n - 1 {
            let laplacian = (old[i + 1] - 2.0 * old[i] + old[i - 1]) / (dx * dx);
            conc[i] += (d * laplacian - decay * old[i]) * dt;
            conc[i] = conc[i].max(0.0);
        }
        if source_idx < n {
            conc[source_idx] += source_rate * dt;
        }
    }
}

pub fn french_flag_model(concentration: f64, t1: f64, t2: f64) -> u8 {
    if concentration > t1 {
        1
    } else if concentration > t2 {
        2
    } else {
        3
    }
}

pub fn bicoid_gradient(c0: f64, length: f64, lambda: f64, x: f64) -> f64 {
    c0 * (-(x / lambda)).exp() * (1.0 - x / length).max(0.0)
}

pub fn positional_information(thresholds: &[f64], gradient: &[f64]) -> Vec<u8> {
    gradient
        .iter()
        .map(|&g| {
            let mut cell_type = 0u8;
            for (i, &t) in thresholds.iter().enumerate() {
                if g > t {
                    cell_type = (i + 1) as u8;
                }
            }
            cell_type
        })
        .collect()
}

pub fn morphogen_gradient_reaction(
    conc: &mut [f64],
    production: &[f64],
    degradation: f64,
    diffusion: f64,
    dx: f64,
    dt: f64,
    steps: usize,
) {
    let n = conc.len();
    for _ in 0..steps {
        let old = conc.to_vec();
        for i in 1..n - 1 {
            let lap = (old[i + 1] - 2.0 * old[i] + old[i - 1]) / (dx * dx);
            let prod = if i < production.len() {
                production[i]
            } else {
                0.0
            };
            conc[i] = (old[i] + (diffusion * lap + prod - degradation * old[i]) * dt).max(0.0);
        }
    }
}

pub fn shh_patterning(distance: f64, concentration: f64, thresholds: &[(f64, u8)]) -> u8 {
    let effective = concentration * (-distance * 0.1).exp();
    let mut cell_type = 0u8;
    for &(thresh, ctype) in thresholds {
        if effective > thresh {
            cell_type = ctype;
        }
    }
    cell_type
}

pub fn morphogen_noise_filtering(signal: &[f64], window: usize) -> Vec<f64> {
    let n = signal.len();
    let mut filtered = Vec::with_capacity(n);
    for i in 0..n {
        let lo = i.saturating_sub(window);
        let hi = (i + window + 1).min(n);
        let avg = signal[lo..hi].iter().sum::<f64>() / (hi - lo) as f64;
        filtered.push(avg);
    }
    filtered
}

pub fn interpretation_delay(concentration_history: &[f64], delay: usize) -> f64 {
    if delay >= concentration_history.len() {
        return 0.0;
    }
    concentration_history[concentration_history.len() - 1 - delay]
}

pub fn wnt_gradient(source_strength: f64, decay_rate: f64, positions: &[f64]) -> Vec<f64> {
    positions
        .iter()
        .map(|&x| source_strength * (-decay_rate * x.abs()).exp())
        .collect()
}
