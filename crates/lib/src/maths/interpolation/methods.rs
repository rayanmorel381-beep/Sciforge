pub fn least_squares_fit(points: &[(f64, f64)], degree: usize) -> Vec<f64> {
    let n = degree + 1;
    let mut ata = vec![vec![0.0; n]; n];
    let mut atb = vec![0.0; n];
    for &(x, y) in points {
        let mut powers = vec![0.0; 2 * n];
        powers[0] = 1.0;
        for k in 1..2 * n {
            powers[k] = powers[k - 1] * x;
        }
        for i in 0..n {
            for j in i..n {
                ata[i][j] += powers[i + j];
                if i != j {
                    ata[j][i] += powers[i + j];
                }
            }
            atb[i] += powers[i] * y;
        }
    }

    let mut aug = vec![vec![0.0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = ata[i][j];
        }
        aug[i][n] = atb[i];
    }
    for col in 0..n {
        let mut max_row = col;
        for row in col + 1..n {
            if aug[row][col].abs() > aug[max_row][col].abs() {
                max_row = row;
            }
        }
        aug.swap(col, max_row);
        let pivot = aug[col][col];
        if pivot.abs() < 1e-30 {
            continue;
        }
        aug[col][col..=n].iter_mut().for_each(|v| *v /= pivot);
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = aug[row][col];
            let src = aug[col][col..=n].to_vec();
            aug[row][col..=n]
                .iter_mut()
                .zip(&src)
                .for_each(|(d, &s)| *d -= factor * s);
        }
    }
    (0..n).map(|i| aug[i][n]).collect()
}

pub fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || data.len() < window {
        return data.to_vec();
    }
    let mut result = Vec::with_capacity(data.len() - window + 1);
    let mut sum: f64 = data[..window].iter().sum();
    result.push(sum / window as f64);
    for i in window..data.len() {
        sum += data[i] - data[i - window];
        result.push(sum / window as f64);
    }
    result
}

pub fn savitzky_golay_5(data: &[f64]) -> Vec<f64> {
    let n = data.len();
    if n < 5 {
        return data.to_vec();
    }
    let mut result = vec![0.0; n];
    result[0] = data[0];
    result[1] = data[1];
    result[n - 2] = data[n - 2];
    result[n - 1] = data[n - 1];
    for i in 2..n - 2 {
        result[i] = (-3.0 * data[i - 2] + 12.0 * data[i - 1] + 17.0 * data[i] + 12.0 * data[i + 1]
            - 3.0 * data[i + 2])
            / 35.0;
    }
    result
}

pub fn exponential_moving_average(data: &[f64], alpha: f64) -> Vec<f64> {
    if data.is_empty() {
        return vec![];
    }
    let mut result = Vec::with_capacity(data.len());
    result.push(data[0]);
    for i in 1..data.len() {
        result.push(alpha * data[i] + (1.0 - alpha) * result[i - 1]);
    }
    result
}

pub fn weighted_moving_average(data: &[f64], weights: &[f64]) -> Vec<f64> {
    let w = weights.len();
    if data.len() < w {
        return data.to_vec();
    }
    let wsum: f64 = weights.iter().sum();
    let mut result = Vec::with_capacity(data.len() - w + 1);
    for i in 0..=data.len() - w {
        let val: f64 = (0..w).map(|j| data[i + j] * weights[j]).sum();
        result.push(val / wsum);
    }
    result
}

pub fn gaussian_kernel_smooth(data: &[f64], xs: &[f64], bandwidth: f64) -> Vec<f64> {
    xs.iter()
        .map(|&x| {
            let mut num = 0.0;
            let mut den = 0.0;
            for (i, &xi) in xs.iter().enumerate().take(data.len()) {
                let w = (-(x - xi).powi(2) / (2.0 * bandwidth * bandwidth)).exp();
                num += w * data[i];
                den += w;
            }
            if den.abs() < 1e-30 { 0.0 } else { num / den }
        })
        .collect()
}

pub fn loess_smooth(xs: &[f64], ys: &[f64], x: f64, span: f64) -> f64 {
    let n = xs.len();
    let k = ((span * n as f64).ceil() as usize).max(2).min(n);
    let mut dists: Vec<(f64, usize)> = xs
        .iter()
        .enumerate()
        .map(|(i, &xi)| ((x - xi).abs(), i))
        .collect();
    dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let max_dist = dists[k - 1].0;
    if max_dist < 1e-30 {
        return ys[dists[0].1];
    }
    let mut sw = 0.0;
    let mut swx = 0.0;
    let mut swx2 = 0.0;
    let mut swy = 0.0;
    let mut swxy = 0.0;
    for dj in &dists[..k] {
        let idx = dj.1;
        let u = dj.0 / max_dist;
        let w = (1.0 - u * u * u).powi(3).max(0.0);
        let xi = xs[idx];
        let yi = ys[idx];
        sw += w;
        swx += w * xi;
        swx2 += w * xi * xi;
        swy += w * yi;
        swxy += w * xi * yi;
    }
    let det = sw * swx2 - swx * swx;
    if det.abs() < 1e-30 {
        return swy / sw;
    }
    let beta0 = (swx2 * swy - swx * swxy) / det;
    let beta1 = (sw * swxy - swx * swy) / det;
    beta0 + beta1 * x
}

pub fn savitzky_golay_7(data: &[f64]) -> Vec<f64> {
    let n = data.len();
    if n < 7 {
        return data.to_vec();
    }
    let mut result = vec![0.0; n];
    result[..3].copy_from_slice(&data[..3]);
    result[n - 3..n].copy_from_slice(&data[n - 3..n]);
    for i in 3..n - 3 {
        result[i] = (-2.0 * data[i - 3]
            + 3.0 * data[i - 2]
            + 6.0 * data[i - 1]
            + 7.0 * data[i]
            + 6.0 * data[i + 1]
            + 3.0 * data[i + 2]
            - 2.0 * data[i + 3])
            / 21.0;
    }
    result
}

pub fn median_filter(data: &[f64], window: usize) -> Vec<f64> {
    let half = window / 2;
    let n = data.len();
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let lo = i.saturating_sub(half);
        let hi = (i + half + 1).min(n);
        let mut window_data: Vec<f64> = data[lo..hi].to_vec();
        window_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = window_data.len() / 2;
        result.push(if window_data.len().is_multiple_of(2) {
            0.5 * (window_data[mid - 1] + window_data[mid])
        } else {
            window_data[mid]
        });
    }
    result
}

pub fn whittaker_smooth(data: &[f64], lambda: f64, order: usize) -> Vec<f64> {
    let n = data.len();
    let mut z = data.to_vec();
    for _ in 0..50 {
        let mut z_new = vec![0.0; n];
        for i in 0..n {
            let mut penalty = 0.0;
            for d in 0..=order {
                if i + d >= n {
                    break;
                }
                let sign = if d % 2 == 0 { 1.0 } else { -1.0 };
                let binom = binomial_coeff(order, d) as f64;
                penalty += sign * binom * z[(i + d).min(n - 1)];
            }
            z_new[i] = (data[i] + lambda * penalty) / (1.0 + lambda);
        }
        z = z_new;
    }
    z
}

fn binomial_coeff(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let mut result = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

pub fn total_variation_denoise(data: &[f64], lambda: f64, iterations: usize) -> Vec<f64> {
    let n = data.len();
    let mut u = data.to_vec();
    for _ in 0..iterations {
        let mut u_new = vec![0.0; n];
        u_new[0] = data[0];
        u_new[n - 1] = data[n - 1];
        for i in 1..n - 1 {
            let grad_forward = u[i + 1] - u[i];
            let grad_backward = u[i] - u[i - 1];
            let div = if grad_forward.abs() > 1e-30 {
                grad_forward / grad_forward.abs()
            } else {
                0.0
            } - if grad_backward.abs() > 1e-30 {
                grad_backward / grad_backward.abs()
            } else {
                0.0
            };
            u_new[i] = u[i] + lambda * (data[i] - u[i]) + div;
        }
        u = u_new;
    }
    u
}
