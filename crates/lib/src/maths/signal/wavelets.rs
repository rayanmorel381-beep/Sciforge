pub fn haar_transform(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    if n < 2 {
        return signal.to_vec();
    }
    let mut data = signal.to_vec();
    let mut len = n;
    while len >= 2 {
        let half = len / 2;
        let mut temp = vec![0.0; len];
        for i in 0..half {
            temp[i] = (data[2 * i] + data[2 * i + 1]) / std::f64::consts::SQRT_2;
            temp[half + i] = (data[2 * i] - data[2 * i + 1]) / std::f64::consts::SQRT_2;
        }
        data[..len].copy_from_slice(&temp[..len]);
        len = half;
    }
    data
}

pub fn haar_inverse(coeffs: &[f64]) -> Vec<f64> {
    let n = coeffs.len();
    if n < 2 {
        return coeffs.to_vec();
    }
    let mut data = coeffs.to_vec();
    let mut len = 2;
    while len <= n {
        let half = len / 2;
        let mut temp = vec![0.0; len];
        for i in 0..half {
            temp[2 * i] = (data[i] + data[half + i]) / std::f64::consts::SQRT_2;
            temp[2 * i + 1] = (data[i] - data[half + i]) / std::f64::consts::SQRT_2;
        }
        data[..len].copy_from_slice(&temp[..len]);
        len *= 2;
    }
    data
}

pub fn db4_scaling() -> [f64; 8] {
    let s3 = 3.0_f64.sqrt();
    let d = 4.0 * std::f64::consts::SQRT_2;
    [
        (1.0 + s3) / d,
        (3.0 + s3) / d,
        (3.0 - s3) / d,
        (1.0 - s3) / d,
        0.0,
        0.0,
        0.0,
        0.0,
    ]
}

pub fn wavelet_energy(detail_coeffs: &[f64]) -> f64 {
    detail_coeffs.iter().map(|c| c * c).sum()
}

pub fn wavelet_entropy(detail_levels: &[Vec<f64>]) -> f64 {
    let energies: Vec<f64> = detail_levels.iter().map(|d| wavelet_energy(d)).collect();
    let total: f64 = energies.iter().sum();
    if total < 1e-30 {
        return 0.0;
    }
    let mut entropy = 0.0;
    for e in &energies {
        let p = e / total;
        if p > 0.0 {
            entropy -= p * p.ln();
        }
    }
    entropy
}

pub fn threshold_hard(coeffs: &[f64], threshold: f64) -> Vec<f64> {
    coeffs
        .iter()
        .map(|&c| if c.abs() >= threshold { c } else { 0.0 })
        .collect()
}

pub fn threshold_soft(coeffs: &[f64], threshold: f64) -> Vec<f64> {
    coeffs
        .iter()
        .map(|&c| {
            if c.abs() < threshold {
                0.0
            } else {
                c.signum() * (c.abs() - threshold)
            }
        })
        .collect()
}

pub fn universal_threshold(n: usize, sigma: f64) -> f64 {
    sigma * (2.0 * (n as f64).ln()).sqrt()
}

pub fn noise_estimate_mad(detail_coeffs: &[f64]) -> f64 {
    let mut abs_vals: Vec<f64> = detail_coeffs.iter().map(|c| c.abs()).collect();
    abs_vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = if abs_vals.is_empty() {
        0.0
    } else if abs_vals.len().is_multiple_of(2) {
        (abs_vals[abs_vals.len() / 2 - 1] + abs_vals[abs_vals.len() / 2]) / 2.0
    } else {
        abs_vals[abs_vals.len() / 2]
    };
    median / 0.6745
}

pub fn db2_scaling() -> [f64; 4] {
    let s3 = 3.0_f64.sqrt();
    let d = 4.0 * std::f64::consts::SQRT_2;
    [
        (1.0 + s3) / d,
        (3.0 + s3) / d,
        (3.0 - s3) / d,
        (1.0 - s3) / d,
    ]
}

pub fn morlet_wavelet(t: f64, sigma: f64) -> f64 {
    let norm = 1.0 / (std::f64::consts::PI.powf(0.25));
    norm * (-0.5 * t * t / (sigma * sigma)).exp() * (5.0 * t / sigma).cos()
}

pub fn mexican_hat_wavelet(t: f64, sigma: f64) -> f64 {
    let s2 = sigma * sigma;
    let t2s2 = t * t / s2;
    let norm = 2.0 / (3.0_f64.sqrt() * std::f64::consts::PI.powf(0.25)) / sigma;
    norm * (1.0 - t2s2) * (-0.5 * t2s2).exp()
}

pub fn continuous_wavelet_transform_morlet(
    signal: &[f64],
    scales: &[f64],
    dt: f64,
) -> Vec<Vec<f64>> {
    let n = signal.len();
    let mut coeffs = vec![vec![0.0; n]; scales.len()];
    for (si, &scale) in scales.iter().enumerate() {
        let norm = 1.0 / scale.sqrt();
        for (b, coeff) in coeffs[si].iter_mut().enumerate() {
            let mut sum = 0.0;
            for (t, &st) in signal.iter().enumerate() {
                let arg = (t as f64 - b as f64) * dt / scale;
                let psi = morlet_wavelet(arg, 1.0);
                sum += st * psi * dt;
            }
            *coeff = sum * norm;
        }
    }
    coeffs
}

pub fn multiresolution_decomposition(signal: &[f64], levels: usize) -> (Vec<f64>, Vec<Vec<f64>>) {
    let mut approx = signal.to_vec();
    let mut details = Vec::with_capacity(levels);
    for _ in 0..levels {
        if approx.len() < 2 {
            break;
        }
        let half = approx.len() / 2;
        let mut a = vec![0.0; half];
        let mut d = vec![0.0; half];
        for i in 0..half {
            a[i] = (approx[2 * i] + approx[2 * i + 1]) / std::f64::consts::SQRT_2;
            d[i] = (approx[2 * i] - approx[2 * i + 1]) / std::f64::consts::SQRT_2;
        }
        details.push(d);
        approx = a;
    }
    (approx, details)
}

pub fn wavelet_reconstruction_haar(approx: &[f64], details: &[Vec<f64>]) -> Vec<f64> {
    let mut current = approx.to_vec();
    for d in details.iter().rev() {
        let n = current.len().min(d.len());
        let mut reconstructed = vec![0.0; 2 * n];
        for i in 0..n {
            reconstructed[2 * i] = (current[i] + d[i]) / std::f64::consts::SQRT_2;
            reconstructed[2 * i + 1] = (current[i] - d[i]) / std::f64::consts::SQRT_2;
        }
        current = reconstructed;
    }
    current
}

pub fn wavelet_shrinkage_denoise(signal: &[f64], levels: usize) -> Vec<f64> {
    let (approx, details) = multiresolution_decomposition(signal, levels);
    let mut thresholded_details = Vec::with_capacity(details.len());
    for (i, d) in details.iter().enumerate() {
        let sigma = noise_estimate_mad(d);
        let thresh = if i == 0 {
            universal_threshold(signal.len(), sigma)
        } else {
            sigma * (2.0 * (d.len() as f64).ln()).sqrt()
        };
        thresholded_details.push(threshold_soft(d, thresh));
    }
    wavelet_reconstruction_haar(&approx, &thresholded_details)
}

pub fn scalogram_energy(cwt_coeffs: &[Vec<f64>]) -> Vec<f64> {
    cwt_coeffs
        .iter()
        .map(|row| row.iter().map(|c| c * c).sum::<f64>())
        .collect()
}
