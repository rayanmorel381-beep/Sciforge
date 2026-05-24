use crate::maths::complex::Complex;
use std::f64::consts::PI;

pub fn fft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    if n <= 1 {
        return input.to_vec();
    }
    assert!(n.is_power_of_two(), "fft requires power-of-two length");

    let mut buf = input.to_vec();
    let mut tmp = vec![Complex::zero(); n];
    fft_recursive(&mut buf, &mut tmp, n, false);
    buf
}

pub fn ifft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    if n <= 1 {
        return input.to_vec();
    }
    assert!(n.is_power_of_two());

    let mut buf = input.to_vec();
    let mut tmp = vec![Complex::zero(); n];
    fft_recursive(&mut buf, &mut tmp, n, true);
    let scale = 1.0 / n as f64;
    buf.iter().map(|c| c.scale(scale)).collect()
}

fn fft_recursive(buf: &mut [Complex], tmp: &mut [Complex], n: usize, inverse: bool) {
    if n == 1 {
        return;
    }
    let half = n / 2;

    for i in 0..half {
        tmp[i] = buf[2 * i];
        tmp[half + i] = buf[2 * i + 1];
    }
    buf[..n].copy_from_slice(&tmp[..n]);

    fft_recursive(&mut buf[..half], &mut tmp[..half], half, inverse);
    fft_recursive(&mut buf[half..n], &mut tmp[half..n], half, inverse);

    let sign = if inverse { 1.0 } else { -1.0 };
    for k in 0..half {
        let angle = sign * 2.0 * PI * k as f64 / n as f64;
        let twiddle = Complex::from_polar(1.0, angle);
        let even = buf[k];
        let odd = buf[half + k] * twiddle;
        buf[k] = even + odd;
        buf[half + k] = even - odd;
    }
}

pub fn fft_real(input: &[f64]) -> Vec<Complex> {
    let complex_input: Vec<Complex> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let n = complex_input.len().next_power_of_two();
    let mut padded = complex_input;
    padded.resize(n, Complex::zero());
    fft(&padded)
}

pub fn power_spectrum(input: &[f64]) -> Vec<f64> {
    fft_real(input).iter().map(|c| c.norm_sq()).collect()
}

pub fn frequency_bins(n: usize, sample_rate: f64) -> Vec<f64> {
    (0..n).map(|k| k as f64 * sample_rate / n as f64).collect()
}

pub fn convolve(a: &[f64], b: &[f64]) -> Vec<f64> {
    let n = (a.len() + b.len() - 1).next_power_of_two();
    let mut fa: Vec<Complex> = a.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let mut fb: Vec<Complex> = b.iter().map(|&x| Complex::new(x, 0.0)).collect();
    fa.resize(n, Complex::zero());
    fb.resize(n, Complex::zero());
    let fa = fft(&fa);
    let fb = fft(&fb);
    let product: Vec<Complex> = fa.iter().zip(&fb).map(|(&a, &b)| a * b).collect();
    let result = ifft(&product);
    result
        .iter()
        .map(|c| c.re)
        .take(a.len() + b.len() - 1)
        .collect()
}

pub fn cross_correlate(a: &[f64], b: &[f64]) -> Vec<f64> {
    let n = (a.len() + b.len() - 1).next_power_of_two();
    let mut fa: Vec<Complex> = a.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let mut fb: Vec<Complex> = b.iter().map(|&x| Complex::new(x, 0.0)).collect();
    fa.resize(n, Complex::zero());
    fb.resize(n, Complex::zero());
    let fa = fft(&fa);
    let fb = fft(&fb);
    let product: Vec<Complex> = fa.iter().zip(&fb).map(|(&a, &b)| a.conj() * b).collect();
    let result = ifft(&product);
    result
        .iter()
        .map(|c| c.re)
        .take(a.len() + b.len() - 1)
        .collect()
}

pub fn autocorrelation(input: &[f64]) -> Vec<f64> {
    cross_correlate(input, input)
}

pub fn magnitude_spectrum(input: &[f64]) -> Vec<f64> {
    fft_real(input).iter().map(|c| c.norm()).collect()
}

pub fn phase_spectrum(input: &[f64]) -> Vec<f64> {
    fft_real(input).iter().map(|c| c.im.atan2(c.re)).collect()
}

pub fn cepstrum(input: &[f64]) -> Vec<f64> {
    let spectrum = fft_real(input);
    let log_mag: Vec<Complex> = spectrum
        .iter()
        .map(|c| Complex::new(c.norm().max(1e-30).ln(), 0.0))
        .collect();
    let n = log_mag.len();
    let result = ifft(&log_mag);
    result.iter().take(n).map(|c| c.re).collect()
}

pub fn spectral_centroid(magnitudes: &[f64], sample_rate: f64) -> f64 {
    let n = magnitudes.len();
    let total: f64 = magnitudes.iter().sum();
    if total < 1e-30 {
        return 0.0;
    }
    let weighted: f64 = magnitudes
        .iter()
        .enumerate()
        .map(|(k, &m)| k as f64 * sample_rate / n as f64 * m)
        .sum();
    weighted / total
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
    magnitudes.len() - 1
}

pub fn spectral_flatness(magnitudes: &[f64]) -> f64 {
    let n = magnitudes.len() as f64;
    if n < 1.0 {
        return 0.0;
    }
    let log_sum: f64 = magnitudes.iter().map(|&m| m.max(1e-30).ln()).sum();
    let geo_mean = (log_sum / n).exp();
    let arith_mean: f64 = magnitudes.iter().sum::<f64>() / n;
    if arith_mean < 1e-30 {
        return 0.0;
    }
    geo_mean / arith_mean
}

pub fn hann_window(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f64 / (n - 1) as f64).cos()))
        .collect()
}

pub fn hamming_window(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| 0.54 - 0.46 * (2.0 * PI * i as f64 / (n - 1) as f64).cos())
        .collect()
}

pub fn blackman_window(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| {
            let t = 2.0 * PI * i as f64 / (n - 1) as f64;
            0.42 - 0.5 * t.cos() + 0.08 * (2.0 * t).cos()
        })
        .collect()
}

pub fn kaiser_window(n: usize, beta: f64) -> Vec<f64> {
    let denom = bessel_i0(beta);
    (0..n)
        .map(|i| {
            let arg = beta * (1.0 - ((2.0 * i as f64 / (n - 1) as f64) - 1.0).powi(2)).sqrt();
            bessel_i0(arg) / denom
        })
        .collect()
}

fn bessel_i0(x: f64) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    for k in 1..25 {
        term *= (x / (2.0 * k as f64)).powi(2);
        sum += term;
    }
    sum
}

pub fn windowed_fft(input: &[f64], window: &[f64]) -> Vec<Complex> {
    let windowed: Vec<Complex> = input
        .iter()
        .zip(window)
        .map(|(&x, &w)| Complex::new(x * w, 0.0))
        .collect();
    let n = windowed.len().next_power_of_two();
    let mut padded = windowed;
    padded.resize(n, Complex::zero());
    fft(&padded)
}

pub fn stft(input: &[f64], window_size: usize, hop_size: usize) -> Vec<Vec<Complex>> {
    let window = hann_window(window_size);
    let mut frames = Vec::new();
    let mut start = 0;
    while start + window_size <= input.len() {
        let frame = &input[start..start + window_size];
        frames.push(windowed_fft(frame, &window));
        start += hop_size;
    }
    frames
}

pub fn zero_pad(input: &[f64], target_len: usize) -> Vec<f64> {
    let mut result = input.to_vec();
    result.resize(target_len, 0.0);
    result
}

pub fn deconvolve(signal: &[f64], kernel: &[f64], reg: f64) -> Vec<f64> {
    let n = (signal.len().max(kernel.len()) * 2).next_power_of_two();
    let mut fs: Vec<Complex> = signal.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let mut fk: Vec<Complex> = kernel.iter().map(|&x| Complex::new(x, 0.0)).collect();
    fs.resize(n, Complex::zero());
    fk.resize(n, Complex::zero());
    let fs = fft(&fs);
    let fk = fft(&fk);
    let result: Vec<Complex> = fs
        .iter()
        .zip(&fk)
        .map(|(&s, &k)| {
            let denom = k.norm_sq() + reg;
            if denom < 1e-30 {
                Complex::zero()
            } else {
                s * k.conj().scale(1.0 / denom)
            }
        })
        .collect();
    let out = ifft(&result);
    out.iter().map(|c| c.re).take(signal.len()).collect()
}

pub fn spectral_bandwidth(magnitudes: &[f64], sample_rate: f64) -> f64 {
    let centroid = spectral_centroid(magnitudes, sample_rate);
    let n = magnitudes.len();
    let total: f64 = magnitudes.iter().sum();
    if total < 1e-30 {
        return 0.0;
    }
    let variance: f64 = magnitudes
        .iter()
        .enumerate()
        .map(|(k, &m)| {
            let freq = k as f64 * sample_rate / n as f64;
            (freq - centroid).powi(2) * m
        })
        .sum();
    (variance / total).sqrt()
}

pub fn spectral_entropy(magnitudes: &[f64]) -> f64 {
    let total: f64 = magnitudes.iter().sum();
    if total < 1e-30 {
        return 0.0;
    }
    let mut entropy = 0.0;
    for &m in magnitudes {
        let p = m / total;
        if p > 1e-30 {
            entropy -= p * p.ln();
        }
    }
    entropy
}
