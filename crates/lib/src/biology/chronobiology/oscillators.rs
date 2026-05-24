pub fn goodwin_oscillator(
    x: f64,
    y: f64,
    z: f64,
    k1: f64,
    k2: f64,
    k3: f64,
    ki: f64,
    n: f64,
) -> (f64, f64, f64) {
    let dx = k1 / (1.0 + (z / ki).powf(n)) - k2 * x;
    let dy = k1 * x - k2 * y;
    let dz = k1 * y - k3 * z;
    (dx, dy, dz)
}

pub fn van_der_pol_circadian(
    x: f64,
    y: f64,
    mu: f64,
    tau: f64,
    light: f64,
    alpha: f64,
) -> (f64, f64) {
    let omega = 2.0 * std::f64::consts::PI / tau;
    let dx = omega * (y + mu * (x - x.powi(3) / 3.0) + alpha * light);
    let dy = -omega * x / mu;
    (dx, dy)
}

pub fn phase_response(phase: f64, light_intensity: f64, sensitivity: f64, tau: f64) -> f64 {
    let omega = 2.0 * std::f64::consts::PI / tau;
    sensitivity * light_intensity * (-phase * omega).sin()
}

pub fn entrainment_range(coupling_strength: f64, intrinsic_period: f64) -> (f64, f64) {
    let half_range = coupling_strength * intrinsic_period / (2.0 * std::f64::consts::PI);
    (intrinsic_period - half_range, intrinsic_period + half_range)
}

pub fn melatonin_profile(t: f64, onset: f64, offset: f64, amplitude: f64) -> f64 {
    if t >= onset && t <= offset {
        let mid = (onset + offset) / 2.0;
        let width = (offset - onset) / 2.0;
        amplitude * (1.0 - ((t - mid) / width).powi(2)).max(0.0)
    } else {
        0.0
    }
}

pub fn desynchrony_index(observed_period: f64, zeitgeber_period: f64) -> f64 {
    (observed_period - zeitgeber_period).abs() / zeitgeber_period
}

pub fn goodwin_simulate(
    x0: f64,
    y0: f64,
    z0: f64,
    k1: f64,
    k2: f64,
    k3: f64,
    ki: f64,
    n: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut x, mut y, mut z) = (x0, y0, z0);
    result.push((x, y, z));
    for _ in 0..steps {
        let (dx, dy, dz) = goodwin_oscillator(x, y, z, k1, k2, k3, ki, n);
        x = (x + dx * dt).max(0.0);
        y = (y + dy * dt).max(0.0);
        z = (z + dz * dt).max(0.0);
        result.push((x, y, z));
    }
    result
}

pub fn kuramoto_order_parameter(phases: &[f64]) -> (f64, f64) {
    let n = phases.len() as f64;
    if n == 0.0 {
        return (0.0, 0.0);
    }
    let cos_sum: f64 = phases.iter().map(|&p| p.cos()).sum();
    let sin_sum: f64 = phases.iter().map(|&p| p.sin()).sum();
    let r = ((cos_sum / n).powi(2) + (sin_sum / n).powi(2)).sqrt();
    let psi = (sin_sum).atan2(cos_sum);
    (r, psi)
}

pub fn kuramoto_step(phases: &mut [f64], frequencies: &[f64], coupling: f64, dt: f64) {
    let n = phases.len();
    let (r, psi) = kuramoto_order_parameter(phases);
    for i in 0..n.min(frequencies.len()) {
        let dphi = frequencies[i] + coupling * r * (psi - phases[i]).sin();
        phases[i] += dphi * dt;
    }
}

pub fn arnolds_tongue_boundary(coupling: f64, detuning: f64) -> bool {
    detuning.abs() < coupling
}

pub fn repressilator(
    a: f64,
    b: f64,
    c: f64,
    alpha: f64,
    alpha0: f64,
    n: f64,
    beta: f64,
) -> (f64, f64, f64) {
    let da = -a + alpha / (1.0 + c.powf(n)) + alpha0;
    let db = -b + alpha / (1.0 + a.powf(n)) + alpha0;
    let dc = -c + alpha / (1.0 + b.powf(n)) + alpha0;
    (da * beta, db * beta, dc * beta)
}

pub fn amplitude_phase_from_timeseries(values: &[f64], period: f64) -> (f64, f64) {
    let n = values.len();
    if n == 0 {
        return (0.0, 0.0);
    }
    let omega = 2.0 * std::f64::consts::PI / period;
    let mut cos_sum = 0.0;
    let mut sin_sum = 0.0;
    for (i, &v) in values.iter().enumerate() {
        let t = i as f64;
        cos_sum += v * (omega * t).cos();
        sin_sum += v * (omega * t).sin();
    }
    let a = 2.0 * cos_sum / n as f64;
    let b = 2.0 * sin_sum / n as f64;
    ((a * a + b * b).sqrt(), b.atan2(a))
}

pub fn phase_diffusion_coefficient(phase_variance: f64, time: f64) -> f64 {
    phase_variance / (2.0 * time)
}

pub fn limit_cycle_stability(floquet_exponent: f64) -> bool {
    floquet_exponent < 0.0
}

pub fn poincare_section_period(crossing_times: &[f64]) -> f64 {
    if crossing_times.len() < 2 {
        return 0.0;
    }
    let n = crossing_times.len() - 1;
    let total: f64 = (1..=n)
        .map(|i| crossing_times[i] - crossing_times[i - 1])
        .sum();
    total / n as f64
}

pub fn detrend_moving_average(data: &[f64], window: usize) -> Vec<f64> {
    let n = data.len();
    if window == 0 || n < window {
        return data.to_vec();
    }
    let half = window / 2;
    (0..n)
        .map(|i| {
            let lo = i.saturating_sub(half);
            let hi = (i + half + 1).min(n);
            let mean = data[lo..hi].iter().sum::<f64>() / (hi - lo) as f64;
            data[i] - mean
        })
        .collect()
}

pub fn instantaneous_frequency(phase_prev: f64, phase_curr: f64, dt: f64) -> f64 {
    let mut dp = phase_curr - phase_prev;
    while dp > std::f64::consts::PI {
        dp -= 2.0 * std::f64::consts::PI;
    }
    while dp < -std::f64::consts::PI {
        dp += 2.0 * std::f64::consts::PI;
    }
    dp / (2.0 * std::f64::consts::PI * dt)
}

pub fn mutual_information_phase(phases1: &[f64], phases2: &[f64], n_bins: usize) -> f64 {
    let n = phases1.len().min(phases2.len());
    if n == 0 || n_bins == 0 {
        return 0.0;
    }
    let bin_width = 2.0 * std::f64::consts::PI / n_bins as f64;
    let mut joint = vec![vec![0usize; n_bins]; n_bins];
    let mut marg1 = vec![0usize; n_bins];
    let mut marg2 = vec![0usize; n_bins];
    for (&p1, &p2) in phases1.iter().zip(phases2.iter()).take(n) {
        let b1 = ((p1.rem_euclid(2.0 * std::f64::consts::PI)) / bin_width) as usize % n_bins;
        let b2 = ((p2.rem_euclid(2.0 * std::f64::consts::PI)) / bin_width) as usize % n_bins;
        joint[b1][b2] += 1;
        marg1[b1] += 1;
        marg2[b2] += 1;
    }
    let nf = n as f64;
    let mut mi = 0.0;
    for (b1, row) in joint.iter().enumerate() {
        for (b2, &count) in row.iter().enumerate() {
            if count > 0 {
                let pjoint = count as f64 / nf;
                let p1 = marg1[b1] as f64 / nf;
                let p2 = marg2[b2] as f64 / nf;
                mi += pjoint * (pjoint / (p1 * p2)).ln();
            }
        }
    }
    mi
}

pub fn stochastic_resonance_snr(signal_power: f64, noise_intensity: f64, threshold: f64) -> f64 {
    let barrier = threshold * threshold / (2.0 * noise_intensity);
    signal_power * (-barrier).exp()
}
