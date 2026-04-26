use super::descriptive::{mean, sample_variance};

pub fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let sx: f64 = x.iter().sum();
    let sy: f64 = y.iter().sum();
    let sxy: f64 = x.iter().zip(y).map(|(xi, yi)| xi * yi).sum();
    let sxx: f64 = x.iter().map(|xi| xi * xi).sum();
    let slope = (n * sxy - sx * sy) / (n * sxx - sx * sx);
    let intercept = (sy - slope * sx) / n;
    (slope, intercept)
}

pub fn standard_error_slope(x: &[f64], y: &[f64]) -> f64 {
    let (slope, intercept) = linear_regression(x, y);
    let n = x.len() as f64;
    let residual_var: f64 = x
        .iter()
        .zip(y)
        .map(|(xi, yi)| (yi - slope * xi - intercept).powi(2))
        .sum::<f64>()
        / (n - 2.0);
    let x_var = sample_variance(x);
    (residual_var / ((n - 1.0) * x_var)).sqrt()
}

pub fn r_squared(x: &[f64], y: &[f64]) -> f64 {
    let (slope, intercept) = linear_regression(x, y);
    let my = mean(y);
    let ss_res: f64 = x
        .iter()
        .zip(y)
        .map(|(xi, yi)| (yi - (slope * xi + intercept)).powi(2))
        .sum();
    let ss_tot: f64 = y.iter().map(|yi| (yi - my).powi(2)).sum();
    1.0 - ss_res / ss_tot
}

pub fn polynomial_regression(x: &[f64], y: &[f64], degree: usize) -> Vec<f64> {
    let n = degree + 1;
    let m = x.len();
    let mut ata = vec![vec![0.0; n]; n];
    let mut atb = vec![0.0; n];

    for k in 0..m {
        let mut powers = vec![0.0; 2 * n];
        powers[0] = 1.0;
        for p in 1..2 * n {
            powers[p] = powers[p - 1] * x[k];
        }
        for i in 0..n {
            for j in i..n {
                ata[i][j] += powers[i + j];
                if i != j {
                    ata[j][i] += powers[i + j];
                }
            }
            atb[i] += powers[i] * y[k];
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

pub fn exponential_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let ln_y: Vec<f64> = y.iter().map(|yi| yi.ln()).collect();
    let (slope, intercept) = linear_regression(x, &ln_y);
    (intercept.exp(), slope)
}

pub fn multiple_linear_regression(x_matrix: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let m = y.len();
    let n = x_matrix[0].len() + 1;
    let mut ata = vec![vec![0.0; n]; n];
    let mut atb = vec![0.0; n];

    for k in 0..m {
        let mut row = vec![1.0];
        row.extend_from_slice(&x_matrix[k]);
        for i in 0..n {
            for j in 0..n {
                ata[i][j] += row[i] * row[j];
            }
            atb[i] += row[i] * y[k];
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

pub fn logarithmic_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let ln_x: Vec<f64> = x.iter().map(|xi| xi.ln()).collect();
    linear_regression(&ln_x, y)
}

pub fn power_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let ln_x: Vec<f64> = x.iter().map(|xi| xi.ln()).collect();
    let ln_y: Vec<f64> = y.iter().map(|yi| yi.ln()).collect();
    let (slope, intercept) = linear_regression(&ln_x, &ln_y);
    (intercept.exp(), slope)
}

pub fn adjusted_r_squared(x: &[f64], y: &[f64], p: usize) -> f64 {
    let r2 = r_squared(x, y);
    let n = x.len() as f64;
    1.0 - (1.0 - r2) * (n - 1.0) / (n - p as f64 - 1.0)
}

pub fn residuals(x: &[f64], y: &[f64]) -> Vec<f64> {
    let (slope, intercept) = linear_regression(x, y);
    x.iter()
        .zip(y)
        .map(|(xi, yi)| yi - (slope * xi + intercept))
        .collect()
}

pub fn sum_of_squared_residuals(x: &[f64], y: &[f64]) -> f64 {
    residuals(x, y).iter().map(|r| r * r).sum()
}

pub fn mean_squared_error(x: &[f64], y: &[f64]) -> f64 {
    sum_of_squared_residuals(x, y) / x.len() as f64
}

pub fn root_mean_squared_error(x: &[f64], y: &[f64]) -> f64 {
    mean_squared_error(x, y).sqrt()
}

pub fn durbin_watson(x: &[f64], y: &[f64]) -> f64 {
    let res = residuals(x, y);
    let n = res.len();
    if n < 2 {
        return 0.0;
    }
    let num: f64 = (1..n).map(|i| (res[i] - res[i - 1]).powi(2)).sum();
    let den: f64 = res.iter().map(|r| r * r).sum();
    if den < 1e-30 {
        return 0.0;
    }
    num / den
}

pub fn leverage_hat(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let mx = mean(x);
    let sxx: f64 = x.iter().map(|xi| (xi - mx).powi(2)).sum();
    x.iter()
        .map(|xi| 1.0 / n as f64 + (xi - mx).powi(2) / sxx)
        .collect()
}

pub fn cook_distance(x: &[f64], y: &[f64]) -> Vec<f64> {
    let res = residuals(x, y);
    let h = leverage_hat(x);
    let mse = mean_squared_error(x, y);
    let p = 2.0;
    res.iter()
        .zip(&h)
        .map(|(r, hi)| r * r * hi / (p * mse * (1.0 - hi).powi(2)))
        .collect()
}

pub fn aic(n: usize, k: usize, ssr: f64) -> f64 {
    let nf = n as f64;
    nf * (ssr / nf).ln() + 2.0 * k as f64
}

pub fn bic(n: usize, k: usize, ssr: f64) -> f64 {
    let nf = n as f64;
    nf * (ssr / nf).ln() + k as f64 * nf.ln()
}

pub fn ridge_regression(x_matrix: &[Vec<f64>], y: &[f64], lambda: f64) -> Vec<f64> {
    let m = y.len();
    let n = x_matrix[0].len() + 1;
    let mut ata = vec![vec![0.0; n]; n];
    let mut atb = vec![0.0; n];

    for k in 0..m {
        let mut row = vec![1.0];
        row.extend_from_slice(&x_matrix[k]);
        for i in 0..n {
            for j in 0..n {
                ata[i][j] += row[i] * row[j];
            }
            atb[i] += row[i] * y[k];
        }
    }

    for (i, ata_i) in ata.iter_mut().enumerate().skip(1) {
        ata_i[i] += lambda;
    }

    gauss_solve_augmented(&ata, &atb)
}

fn gauss_solve_augmented(ata: &[Vec<f64>], atb: &[f64]) -> Vec<f64> {
    let n = atb.len();
    let mut aug = vec![vec![0.0; n + 1]; n];
    for (i, aug_i) in aug.iter_mut().enumerate() {
        aug_i[..n].copy_from_slice(&ata[i]);
        aug_i[n] = atb[i];
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
