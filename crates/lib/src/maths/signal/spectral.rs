pub fn power_spectral_density(signal: &[f64], sample_rate: f64) -> Vec<f64> {
    let n = signal.len();
    let (re, im) = dft(signal);
    let dt = 1.0 / sample_rate;
    re.iter()
        .zip(im.iter())
        .map(|(r, i)| (r * r + i * i) * dt / n as f64)
        .collect()
}

pub fn dft(signal: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = signal.len();
    let mut re = vec![0.0; n];
    let mut im = vec![0.0; n];
    for k in 0..n {
        for (j, &s) in signal.iter().enumerate() {
            let angle = -2.0 * std::f64::consts::PI * k as f64 * j as f64 / n as f64;
            re[k] += s * angle.cos();
            im[k] += s * angle.sin();
        }
    }
    (re, im)
}

pub fn idft(re: &[f64], im: &[f64]) -> Vec<f64> {
    let n = re.len();
    let mut out = vec![0.0; n];
    for (j, oj) in out.iter_mut().enumerate() {
        for (k, (&rk, &ik)) in re.iter().zip(im.iter()).enumerate() {
            let angle = 2.0 * std::f64::consts::PI * k as f64 * j as f64 / n as f64;
            *oj += rk * angle.cos() - ik * angle.sin();
        }
        *oj /= n as f64;
    }
    out
}

pub fn magnitude_spectrum(re: &[f64], im: &[f64]) -> Vec<f64> {
    re.iter()
        .zip(im.iter())
        .map(|(r, i)| (r * r + i * i).sqrt())
        .collect()
}

pub fn phase_spectrum(re: &[f64], im: &[f64]) -> Vec<f64> {
    re.iter().zip(im.iter()).map(|(r, i)| i.atan2(*r)).collect()
}

pub fn frequency_bins(n: usize, sample_rate: f64) -> Vec<f64> {
    (0..n).map(|k| k as f64 * sample_rate / n as f64).collect()
}

pub fn spectral_centroid(magnitudes: &[f64], frequencies: &[f64]) -> f64 {
    let weighted_sum: f64 = magnitudes
        .iter()
        .zip(frequencies.iter())
        .map(|(m, f)| m * f)
        .sum();
    let total: f64 = magnitudes.iter().sum();
    if total < 1e-30 {
        0.0
    } else {
        weighted_sum / total
    }
}

pub fn spectral_bandwidth(magnitudes: &[f64], frequencies: &[f64], centroid: f64) -> f64 {
    let total: f64 = magnitudes.iter().sum();
    if total < 1e-30 {
        return 0.0;
    }
    let variance: f64 = magnitudes
        .iter()
        .zip(frequencies.iter())
        .map(|(m, f)| m * (f - centroid).powi(2))
        .sum::<f64>()
        / total;
    variance.sqrt()
}

pub fn spectral_rolloff(magnitudes: &[f64], threshold: f64) -> usize {
    let total: f64 = magnitudes.iter().sum();
    let target = total * threshold;
    let mut cumsum = 0.0;
    for (i, &m) in magnitudes.iter().enumerate() {
        cumsum += m;
        if cumsum >= target {
            return i;
        }
    }
    magnitudes.len().saturating_sub(1)
}

pub fn windowed_signal(signal: &[f64], window: &[f64]) -> Vec<f64> {
    signal
        .iter()
        .zip(window.iter())
        .map(|(s, w)| s * w)
        .collect()
}

pub fn hanning_window(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64).cos()))
        .collect()
}

pub fn hamming_window(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| 0.54 - 0.46 * (2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64).cos())
        .collect()
}

pub fn blackman_window(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| {
            let x = 2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64;
            0.42 - 0.5 * x.cos() + 0.08 * (2.0 * x).cos()
        })
        .collect()
}

pub fn kaiser_window(n: usize, beta: f64) -> Vec<f64> {
    let denom = bessel_i0_internal(beta);
    (0..n)
        .map(|i| {
            let x = 2.0 * i as f64 / (n - 1) as f64 - 1.0;
            bessel_i0_internal(beta * (1.0 - x * x).sqrt()) / denom
        })
        .collect()
}

fn bessel_i0_internal(x: f64) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    for k in 1..25 {
        term *= (x / 2.0) * (x / 2.0) / (k as f64 * k as f64);
        sum += term;
    }
    sum
}

pub fn stft(signal: &[f64], window_size: usize, hop_size: usize) -> Vec<(Vec<f64>, Vec<f64>)> {
    let window = hanning_window(window_size);
    let mut frames = Vec::new();
    let mut pos = 0;
    while pos + window_size <= signal.len() {
        let frame: Vec<f64> = (0..window_size)
            .map(|i| signal[pos + i] * window[i])
            .collect();
        frames.push(dft(&frame));
        pos += hop_size;
    }
    frames
}

pub fn cross_spectral_density(signal_a: &[f64], signal_b: &[f64], sample_rate: f64) -> Vec<f64> {
    let n = signal_a.len().min(signal_b.len());
    let (ra, ia) = dft(&signal_a[..n]);
    let (rb, ib) = dft(&signal_b[..n]);
    let dt = 1.0 / sample_rate;
    (0..n)
        .map(|k| {
            let cross_re = ra[k] * rb[k] + ia[k] * ib[k];
            let cross_im = ia[k] * rb[k] - ra[k] * ib[k];
            (cross_re * cross_re + cross_im * cross_im).sqrt() * dt / n as f64
        })
        .collect()
}

pub fn spectral_flatness(magnitudes: &[f64]) -> f64 {
    let n = magnitudes.len() as f64;
    if n < 1.0 {
        return 0.0;
    }
    let log_sum: f64 = magnitudes
        .iter()
        .filter(|&&m| m > 0.0)
        .map(|m| m.ln())
        .sum();
    let geometric_mean = (log_sum / n).exp();
    let arithmetic_mean: f64 = magnitudes.iter().sum::<f64>() / n;
    if arithmetic_mean < 1e-30 {
        0.0
    } else {
        geometric_mean / arithmetic_mean
    }
}

pub fn spectral_flux(current: &[f64], previous: &[f64]) -> f64 {
    current
        .iter()
        .zip(previous.iter())
        .map(|(&c, &p)| {
            let diff = c - p;
            if diff > 0.0 { diff * diff } else { 0.0 }
        })
        .sum::<f64>()
        .sqrt()
}

pub fn bark_scale(freq: f64) -> f64 {
    13.0 * (0.00076 * freq).atan() + 3.5 * (freq / 7500.0).powi(2).atan()
}

pub fn mel_scale(freq: f64) -> f64 {
    2595.0 * (1.0 + freq / 700.0).log10()
}

pub fn inverse_mel(mel: f64) -> f64 {
    700.0 * (10.0_f64.powf(mel / 2595.0) - 1.0)
}
