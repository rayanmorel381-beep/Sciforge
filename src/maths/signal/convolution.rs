pub fn convolve(a: &[f64], b: &[f64]) -> Vec<f64> {
    let n = a.len() + b.len() - 1;
    let mut out = vec![0.0; n];
    for i in 0..a.len() {
        for j in 0..b.len() {
            out[i + j] += a[i] * b[j];
        }
    }
    out
}

pub fn cross_correlate(a: &[f64], b: &[f64]) -> Vec<f64> {
    let n = a.len() + b.len() - 1;
    let mut out = vec![0.0; n];
    for i in 0..a.len() {
        for j in 0..b.len() {
            out[i + b.len() - 1 - j] += a[i] * b[j];
        }
    }
    out
}

pub fn autocorrelation(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    let mut out = vec![0.0; n];
    for lag in 0..n {
        let mut s = 0.0;
        for i in 0..n - lag {
            s += signal[i] * signal[i + lag];
        }
        out[lag] = s;
    }
    out
}

pub fn deconvolve(signal: &[f64], kernel: &[f64]) -> Vec<f64> {
    if kernel.is_empty() || kernel[0].abs() < 1e-30 {
        return vec![];
    }
    let out_len = signal.len() - kernel.len() + 1;
    if out_len == 0 {
        return vec![];
    }
    let mut out = vec![0.0; out_len];
    let mut remainder = signal.to_vec();
    for i in 0..out_len {
        out[i] = remainder[i] / kernel[0];
        for j in 0..kernel.len() {
            remainder[i + j] -= out[i] * kernel[j];
        }
    }
    out
}

pub fn matched_filter(signal: &[f64], template: &[f64]) -> Vec<f64> {
    let reversed: Vec<f64> = template.iter().rev().cloned().collect();
    convolve(signal, &reversed)
}

pub fn downsample(signal: &[f64], factor: usize) -> Vec<f64> {
    signal.iter().step_by(factor).cloned().collect()
}

pub fn upsample(signal: &[f64], factor: usize) -> Vec<f64> {
    let mut out = vec![0.0; (signal.len() - 1) * factor + 1];
    for (i, &s) in signal.iter().enumerate() {
        out[i * factor] = s;
    }
    out
}

pub fn zero_pad(signal: &[f64], total_length: usize) -> Vec<f64> {
    let mut out = signal.to_vec();
    out.resize(total_length, 0.0);
    out
}

pub fn convolve_same(a: &[f64], b: &[f64]) -> Vec<f64> {
    let full = convolve(a, b);
    let offset = b.len() / 2;
    full[offset..offset + a.len()].to_vec()
}

pub fn convolve_valid(a: &[f64], b: &[f64]) -> Vec<f64> {
    if a.len() < b.len() {
        return vec![];
    }
    let out_len = a.len() - b.len() + 1;
    let mut out = vec![0.0; out_len];
    for i in 0..out_len {
        let mut s = 0.0;
        for j in 0..b.len() {
            s += a[i + j] * b[b.len() - 1 - j];
        }
        out[i] = s;
    }
    out
}

pub fn circular_convolution(a: &[f64], b: &[f64]) -> Vec<f64> {
    let n = a.len().max(b.len());
    let mut ap = a.to_vec();
    let mut bp = b.to_vec();
    ap.resize(n, 0.0);
    bp.resize(n, 0.0);
    let mut out = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            out[i] += ap[j] * bp[(i + n - j) % n];
        }
    }
    out
}

pub fn overlap_add(signal: &[f64], kernel: &[f64], block_size: usize) -> Vec<f64> {
    let m = kernel.len();
    let out_len = signal.len() + m - 1;
    let mut output = vec![0.0; out_len];
    let mut pos = 0;
    while pos < signal.len() {
        let end = (pos + block_size).min(signal.len());
        let block = &signal[pos..end];
        let segment = convolve(block, kernel);
        for (i, &v) in segment.iter().enumerate() {
            if pos + i < out_len {
                output[pos + i] += v;
            }
        }
        pos += block_size;
    }
    output
}

pub fn normalized_cross_correlation(a: &[f64], b: &[f64]) -> Vec<f64> {
    let corr = cross_correlate(a, b);
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    let denom = norm_a * norm_b;
    if denom < 1e-30 {
        return corr;
    }
    corr.iter().map(|&c| c / denom).collect()
}

pub fn convolution_2d(image: &[Vec<f64>], kernel: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let ny = image.len();
    let nx = image[0].len();
    let ky = kernel.len();
    let kx = kernel[0].len();
    let hy = ky / 2;
    let hx = kx / 2;
    let mut out = vec![vec![0.0; nx]; ny];
    for (j, out_j) in out.iter_mut().enumerate() {
        for (i, out_ji) in out_j.iter_mut().enumerate() {
            let mut s = 0.0;
            for (kj, kernel_row) in kernel.iter().enumerate() {
                for (ki, &kval) in kernel_row.iter().enumerate() {
                    let sj = j as i64 + kj as i64 - hy as i64;
                    let si = i as i64 + ki as i64 - hx as i64;
                    if sj >= 0 && sj < ny as i64 && si >= 0 && si < nx as i64 {
                        s += image[sj as usize][si as usize] * kval;
                    }
                }
            }
            *out_ji = s;
        }
    }
    out
}

pub fn wiener_deconvolution(signal: &[f64], kernel: &[f64], noise_power: f64) -> Vec<f64> {
    let n = signal.len();
    let (sr, si) = dft_internal(signal);
    let mut kp = kernel.to_vec();
    kp.resize(n, 0.0);
    let (kr, ki) = dft_internal(&kp);
    let mut out_re = vec![0.0; n];
    let mut out_im = vec![0.0; n];
    for i in 0..n {
        let k_mag2 = kr[i] * kr[i] + ki[i] * ki[i];
        let wiener = k_mag2 / (k_mag2 + noise_power);
        let inv_k_re = kr[i] / (k_mag2 + 1e-30);
        let inv_k_im = -ki[i] / (k_mag2 + 1e-30);
        out_re[i] = wiener * (sr[i] * inv_k_re - si[i] * inv_k_im);
        out_im[i] = wiener * (sr[i] * inv_k_im + si[i] * inv_k_re);
    }
    idft_internal(&out_re, &out_im)
}

fn dft_internal(signal: &[f64]) -> (Vec<f64>, Vec<f64>) {
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

fn idft_internal(re: &[f64], im: &[f64]) -> Vec<f64> {
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
