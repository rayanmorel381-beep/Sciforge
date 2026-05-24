use crate::constants::{SAVITZKY_GOLAY_5_COEFFS, SAVITZKY_GOLAY_5_NORM};

pub fn low_pass_rc(signal: &[f64], dt: f64, rc: f64) -> Vec<f64> {
    let alpha = dt / (rc + dt);
    let mut out = vec![0.0; signal.len()];
    if signal.is_empty() {
        return out;
    }
    out[0] = alpha * signal[0];
    for i in 1..signal.len() {
        out[i] = alpha * signal[i] + (1.0 - alpha) * out[i - 1];
    }
    out
}

pub fn high_pass_rc(signal: &[f64], dt: f64, rc: f64) -> Vec<f64> {
    let alpha = rc / (rc + dt);
    let mut out = vec![0.0; signal.len()];
    if signal.is_empty() {
        return out;
    }
    out[0] = signal[0];
    for i in 1..signal.len() {
        out[i] = alpha * (out[i - 1] + signal[i] - signal[i - 1]);
    }
    out
}

pub fn moving_average(signal: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || signal.is_empty() {
        return signal.to_vec();
    }
    let n = signal.len();
    let mut out = vec![0.0; n];
    let mut sum = 0.0;
    for i in 0..n {
        sum += signal[i];
        if i >= window {
            sum -= signal[i - window];
        }
        let count = (i + 1).min(window);
        out[i] = sum / count as f64;
    }
    out
}

pub fn exponential_moving_average(signal: &[f64], alpha: f64) -> Vec<f64> {
    let mut out = vec![0.0; signal.len()];
    if signal.is_empty() {
        return out;
    }
    out[0] = signal[0];
    for i in 1..signal.len() {
        out[i] = alpha * signal[i] + (1.0 - alpha) * out[i - 1];
    }
    out
}

pub fn median_filter(signal: &[f64], window: usize) -> Vec<f64> {
    let n = signal.len();
    let half = window / 2;
    let mut out = vec![0.0; n];
    for (i, oi) in out.iter_mut().enumerate() {
        let start = i.saturating_sub(half);
        let end = (i + half + 1).min(n);
        let mut w: Vec<f64> = signal[start..end].to_vec();
        w.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        *oi = w[w.len() / 2];
    }
    out
}

pub fn butterworth_gain(freq: f64, cutoff: f64, order: u32) -> f64 {
    1.0 / (1.0 + (freq / cutoff).powi(2 * order as i32)).sqrt()
}

pub fn chebyshev_gain(freq: f64, cutoff: f64, order: u32, ripple_db: f64) -> f64 {
    let eps = (10.0_f64.powf(ripple_db / 10.0) - 1.0).sqrt();
    let x = freq / cutoff;
    let tn = chebyshev_poly(order, x);
    1.0 / (1.0 + eps * eps * tn * tn).sqrt()
}

fn chebyshev_poly(n: u32, x: f64) -> f64 {
    if x.abs() <= 1.0 {
        (n as f64 * x.acos()).cos()
    } else {
        (n as f64 * x.acosh()).cosh()
    }
}

pub fn savitzky_golay_5(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    if n < 5 {
        return signal.to_vec();
    }
    let mut out = signal.to_vec();
    let coeffs = SAVITZKY_GOLAY_5_COEFFS;
    let norm = SAVITZKY_GOLAY_5_NORM;
    for i in 2..n - 2 {
        let mut s = 0.0;
        for (j, &c) in coeffs.iter().enumerate() {
            s += c * signal[i + j - 2];
        }
        out[i] = s / norm;
    }
    out
}

pub fn notch_filter(signal: &[f64], dt: f64, freq: f64, bandwidth: f64) -> Vec<f64> {
    let n = signal.len();
    let mut out = vec![0.0; n];
    if n < 3 {
        return signal.to_vec();
    }
    let w0 = 2.0 * std::f64::consts::PI * freq * dt;
    let bw = 2.0 * std::f64::consts::PI * bandwidth * dt;
    let r = 1.0 - bw / 2.0;
    let a1 = -2.0 * r * w0.cos();
    let a2 = r * r;
    let b0 = 1.0;
    let b1 = -2.0 * w0.cos();
    let b2 = 1.0;
    out[0] = signal[0];
    out[1] = signal[1];
    for i in 2..n {
        out[i] = b0 * signal[i] + b1 * signal[i - 1] + b2 * signal[i - 2]
            - a1 * out[i - 1]
            - a2 * out[i - 2];
    }
    out
}

pub fn band_pass_filter(signal: &[f64], dt: f64, low_freq: f64, high_freq: f64) -> Vec<f64> {
    let low_passed = low_pass_rc(signal, dt, 1.0 / (2.0 * std::f64::consts::PI * high_freq));
    high_pass_rc(
        &low_passed,
        dt,
        1.0 / (2.0 * std::f64::consts::PI * low_freq),
    )
}

pub fn gaussian_filter(signal: &[f64], sigma: f64) -> Vec<f64> {
    let radius = (3.0 * sigma).ceil() as usize;
    let kernel_size = 2 * radius + 1;
    let mut kernel = vec![0.0; kernel_size];
    let mut sum = 0.0;
    for (i, ki) in kernel.iter_mut().enumerate() {
        let x = i as f64 - radius as f64;
        *ki = (-x * x / (2.0 * sigma * sigma)).exp();
        sum += *ki;
    }
    for k in &mut kernel {
        *k /= sum;
    }
    let n = signal.len();
    let mut out = vec![0.0; n];
    for (i, oi) in out.iter_mut().enumerate() {
        let mut val = 0.0;
        for (j, &kj) in kernel.iter().enumerate() {
            let idx = i as i64 + j as i64 - radius as i64;
            let idx = idx.max(0).min(n as i64 - 1) as usize;
            val += signal[idx] * kj;
        }
        *oi = val;
    }
    out
}

pub fn wiener_filter_frequency(signal_power: &[f64], noise_power: &[f64]) -> Vec<f64> {
    signal_power
        .iter()
        .zip(noise_power.iter())
        .map(|(&s, &n)| if s + n < 1e-30 { 0.0 } else { s / (s + n) })
        .collect()
}

pub fn kalman_filter_1d(
    measurements: &[f64],
    process_noise: f64,
    measurement_noise: f64,
    initial_estimate: f64,
) -> Vec<f64> {
    let mut estimates = vec![0.0; measurements.len()];
    let mut x = initial_estimate;
    let mut p = 1.0;
    for (est, &mi) in estimates.iter_mut().zip(measurements.iter()) {
        p += process_noise;
        let k = p / (p + measurement_noise);
        x += k * (mi - x);
        p *= 1.0 - k;
        *est = x;
    }
    estimates
}

pub fn adaptive_lms_filter(
    input: &[f64],
    desired: &[f64],
    filter_length: usize,
    mu: f64,
) -> Vec<f64> {
    let n = input.len().min(desired.len());
    let mut w = vec![0.0; filter_length];
    let mut output = vec![0.0; n];
    for (i, (oi, &di)) in output.iter_mut().zip(desired.iter()).enumerate() {
        let mut y = 0.0;
        for j in 0..filter_length {
            if i >= j {
                y += w[j] * input[i - j];
            }
        }
        *oi = y;
        let error = di - y;
        for j in 0..filter_length {
            if i >= j {
                w[j] += mu * error * input[i - j];
            }
        }
    }
    output
}

pub fn bessel_gain(freq: f64, cutoff: f64, order: u32) -> f64 {
    let s = freq / cutoff;
    let denom = bessel_poly(order, s);
    1.0 / denom
}

fn bessel_poly(n: u32, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return (1.0 + x * x).sqrt();
    }
    let mut b_prev2 = 1.0;
    let mut b_prev1 = (1.0 + x * x).sqrt();
    for k in 2..=n {
        let b = ((2 * k - 1) as f64 * b_prev1 + x * x * b_prev2).sqrt();
        b_prev2 = b_prev1;
        b_prev1 = b;
    }
    b_prev1
}

pub fn weighted_moving_average(signal: &[f64], weights: &[f64]) -> Vec<f64> {
    let n = signal.len();
    let wn = weights.len();
    let w_sum: f64 = weights.iter().sum();
    let mut out = vec![0.0; n];
    for (i, oi) in out.iter_mut().enumerate() {
        let mut val = 0.0;
        for (j, &wj) in weights.iter().enumerate() {
            let idx = (i + j).saturating_sub(wn / 2);
            let idx = idx.min(n - 1);
            val += signal[idx] * wj;
        }
        *oi = val / w_sum;
    }
    out
}

pub fn derivative_filter(signal: &[f64], dt: f64) -> Vec<f64> {
    let n = signal.len();
    let mut out = vec![0.0; n];
    if n < 2 {
        return out;
    }
    out[0] = (signal[1] - signal[0]) / dt;
    for i in 1..n - 1 {
        out[i] = (signal[i + 1] - signal[i - 1]) / (2.0 * dt);
    }
    out[n - 1] = (signal[n - 1] - signal[n - 2]) / dt;
    out
}
