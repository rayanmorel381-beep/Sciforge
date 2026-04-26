use super::radix2::{fft, ifft};
use crate::maths::complex::Complex;
use std::f64::consts::PI;

pub fn bluestein_fft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    if n <= 1 {
        return input.to_vec();
    }

    let m = (2 * n - 1).next_power_of_two();

    let chirp: Vec<Complex> = (0..n)
        .map(|k| {
            let angle = -PI * (k * k) as f64 / n as f64;
            Complex::from_polar(1.0, angle)
        })
        .collect();

    let mut a = vec![Complex::zero(); m];
    for k in 0..n {
        a[k] = input[k] * chirp[k];
    }

    let mut b = vec![Complex::zero(); m];
    b[0] = chirp[0].conj();
    for k in 1..n {
        let c = chirp[k].conj();
        b[k] = c;
        b[m - k] = c;
    }

    let fa = fft(&a);
    let fb = fft(&b);
    let product: Vec<Complex> = fa.iter().zip(&fb).map(|(&a, &b)| a * b).collect();
    let conv = ifft(&product);

    (0..n).map(|k| conv[k] * chirp[k]).collect()
}

pub fn bluestein_ifft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    let conj_input: Vec<Complex> = input.iter().map(|c| c.conj()).collect();
    let result = bluestein_fft(&conj_input);
    let scale = 1.0 / n as f64;
    result.iter().map(|c| c.conj().scale(scale)).collect()
}

pub fn fft_arbitrary(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    if n.is_power_of_two() {
        fft(input)
    } else {
        bluestein_fft(input)
    }
}

pub fn ifft_arbitrary(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    if n.is_power_of_two() {
        ifft(input)
    } else {
        bluestein_ifft(input)
    }
}

pub fn goertzel(input: &[f64], freq_bin: f64) -> Complex {
    let n = input.len();
    let w = 2.0 * PI * freq_bin / n as f64;
    let coeff = 2.0 * w.cos();
    let mut s0 = 0.0;
    let mut s1 = 0.0;
    let mut s2;
    for &x in input {
        s2 = s1;
        s1 = s0;
        s0 = x + coeff * s1 - s2;
    }
    Complex::new(s0 - s1 * w.cos(), s1 * w.sin())
}

pub fn goertzel_mag(input: &[f64], freq_bin: f64) -> f64 {
    goertzel(input, freq_bin).norm()
}

pub fn chirp_z_transform(input: &[Complex], m: usize, w: Complex, a: Complex) -> Vec<Complex> {
    let n = input.len();
    let l = (n + m - 1).next_power_of_two();

    let chirp_n: Vec<Complex> = (0..n)
        .map(|k| {
            let angle = -PI * (k * k) as f64 / n as f64;
            let wk = Complex::from_polar(w.norm().powf(k as f64), angle);
            let ak_inv =
                Complex::from_polar(a.norm().powf(-(k as f64)), -(k as f64) * a.im.atan2(a.re));
            input[k] * ak_inv * wk
        })
        .collect();

    let mut yn = vec![Complex::zero(); l];
    yn[..n].copy_from_slice(&chirp_n);

    let mut h = vec![Complex::zero(); l];
    for k in 0..m.max(n) {
        let angle = PI * (k * k) as f64 / n as f64;
        let val = Complex::from_polar(w.norm().powf(-(k as f64)), angle);
        if k < m {
            h[k] = val;
        }
        if k > 0 && k < n {
            h[l - k] = val;
        }
    }

    let fy = fft(&yn);
    let fh = fft(&h);
    let product: Vec<Complex> = fy.iter().zip(&fh).map(|(&a, &b)| a * b).collect();
    let conv = ifft(&product);

    (0..m)
        .map(|k| {
            let angle = -PI * (k * k) as f64 / n as f64;
            conv[k] * Complex::from_polar(w.norm().powf(k as f64), angle)
        })
        .collect()
}

pub fn fft_shift(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    let half = n / 2;
    let mut result = vec![Complex::zero(); n];
    for (i, ri) in result.iter_mut().enumerate() {
        *ri = input[(i + half) % n];
    }
    result
}

pub fn ifft_shift(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    let half = n.div_ceil(2);
    let mut result = vec![Complex::zero(); n];
    for (i, ri) in result.iter_mut().enumerate() {
        *ri = input[(i + half) % n];
    }
    result
}

pub fn hilbert_transform(input: &[f64]) -> Vec<Complex> {
    let n = input.len().next_power_of_two();
    let mut c: Vec<Complex> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
    c.resize(n, Complex::zero());
    let mut spectrum = fft(&c);
    let half = n / 2;
    for sk in &mut spectrum[1..half] {
        *sk = sk.scale(2.0);
    }
    for sk in &mut spectrum[(half + 1)..] {
        *sk = Complex::zero();
    }
    let result = ifft(&spectrum);
    result[..input.len()].to_vec()
}

pub fn analytic_signal(input: &[f64]) -> Vec<Complex> {
    hilbert_transform(input)
}

pub fn instantaneous_frequency(input: &[f64], sample_rate: f64) -> Vec<f64> {
    let analytic = hilbert_transform(input);
    let phases: Vec<f64> = analytic.iter().map(|c| c.im.atan2(c.re)).collect();
    let mut freq = vec![0.0; phases.len()];
    for i in 1..phases.len() {
        let mut dp = phases[i] - phases[i - 1];
        while dp > PI {
            dp -= 2.0 * PI;
        }
        while dp < -PI {
            dp += 2.0 * PI;
        }
        freq[i] = dp * sample_rate / (2.0 * PI);
    }
    freq
}

pub fn sliding_dft(input: &[f64], k: usize, window: usize) -> Vec<Complex> {
    let n = window;
    let w = Complex::from_polar(1.0, 2.0 * PI * k as f64 / n as f64);
    let w_inv = Complex::from_polar(1.0, -2.0 * PI * k as f64 / n as f64);
    let mut result = Vec::new();
    if input.len() < n {
        return result;
    }
    let mut x_k = Complex::zero();
    for &inp in &input[..n] {
        x_k = x_k * w + Complex::new(inp, 0.0);
    }
    result.push(x_k);
    let wn = w.scale(1.0);
    for i in n..input.len() {
        let old = Complex::new(input[i - n], 0.0);
        let new_val = Complex::new(input[i], 0.0);
        x_k = (x_k - old) * wn + new_val * w_inv;
        result.push(x_k);
    }
    result
}
