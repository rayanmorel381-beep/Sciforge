pub fn normal_equations(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    let mut ata = vec![vec![0.0; n]; n];
    let mut atb = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            for ak in a.iter() {
                ata[i][j] += ak[i] * ak[j];
            }
        }
        for k in 0..m {
            atb[i] += a[k][i] * b[k];
        }
    }
    gauss_solve(&ata, &atb)
}

fn gauss_solve(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut aug = vec![vec![0.0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = a[i][j];
        }
        aug[i][n] = b[i];
    }
    for col in 0..n {
        let mut max_row = col;
        for row in (col + 1)..n {
            if aug[row][col].abs() > aug[max_row][col].abs() {
                max_row = row;
            }
        }
        aug.swap(col, max_row);
        let pivot = aug[col][col];
        if pivot.abs() < 1e-30 {
            continue;
        }
        for row in (col + 1)..n {
            let factor = aug[row][col] / pivot;
            let (top, bot) = aug.split_at_mut(row);
            for (d, &s) in bot[0][col..=n].iter_mut().zip(&top[col][col..=n]) {
                *d -= factor * s;
            }
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = 0.0;
        for (&aij, &xj) in aug[i][(i + 1)..n].iter().zip(&x[(i + 1)..]) {
            sum += aij * xj;
        }
        x[i] = (aug[i][n] - sum) / aug[i][i];
    }
    x
}

pub fn residual(a: &[Vec<f64>], x: &[f64], b: &[f64]) -> Vec<f64> {
    let m = a.len();
    let mut r = vec![0.0; m];
    for i in 0..m {
        let mut ax = 0.0;
        for (&aij, &xj) in a[i].iter().zip(x.iter()) {
            ax += aij * xj;
        }
        r[i] = b[i] - ax;
    }
    r
}

pub fn residual_norm(a: &[Vec<f64>], x: &[f64], b: &[f64]) -> f64 {
    let r = residual(a, x, b);
    r.iter().map(|v| v * v).sum::<f64>().sqrt()
}

pub fn r_squared(y_actual: &[f64], y_predicted: &[f64]) -> f64 {
    let mean = y_actual.iter().sum::<f64>() / y_actual.len() as f64;
    let ss_tot: f64 = y_actual.iter().map(|&y| (y - mean).powi(2)).sum();
    let ss_res: f64 = y_actual
        .iter()
        .zip(y_predicted.iter())
        .map(|(&a, &p)| (a - p).powi(2))
        .sum();
    1.0 - ss_res / ss_tot.max(1e-30)
}

pub fn polynomial_fit(x: &[f64], y: &[f64], degree: usize) -> Vec<f64> {
    let m = x.len();
    let n = degree + 1;
    let a: Vec<Vec<f64>> = (0..m)
        .map(|i| (0..n).map(|j| x[i].powi(j as i32)).collect())
        .collect();
    normal_equations(&a, y)
}

pub fn weighted_least_squares(a: &[Vec<f64>], b: &[f64], w: &[f64]) -> Vec<f64> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    let mut ata = vec![vec![0.0; n]; n];
    let mut atb = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..m {
                ata[i][j] += w[k] * a[k][i] * a[k][j];
            }
        }
        for k in 0..m {
            atb[i] += w[k] * a[k][i] * b[k];
        }
    }
    gauss_solve(&ata, &atb)
}

pub fn tikhonov_regularization(a: &[Vec<f64>], b: &[f64], lambda: f64) -> Vec<f64> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    let mut ata = vec![vec![0.0; n]; n];
    let mut atb = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            for ak in a.iter() {
                ata[i][j] += ak[i] * ak[j];
            }
        }
        ata[i][i] += lambda;
        for k in 0..m {
            atb[i] += a[k][i] * b[k];
        }
    }
    gauss_solve(&ata, &atb)
}

pub fn total_least_squares(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    let aug: Vec<Vec<f64>> = (0..m)
        .map(|i| {
            let mut row = a[i].clone();
            row.push(b[i]);
            row
        })
        .collect();
    let mut ata = vec![vec![0.0; n + 1]; n + 1];
    for (i, ata_i) in ata.iter_mut().enumerate() {
        for (j, ata_ij) in ata_i.iter_mut().enumerate() {
            for aug_k in aug.iter() {
                *ata_ij += aug_k[i] * aug_k[j];
            }
        }
    }
    let dim = n + 1;
    let mut v = vec![1.0 / (dim as f64).sqrt(); dim];
    for _ in 0..200 {
        let mut shifted = ata.clone();
        let lambda_est: f64 = {
            let mut av = vec![0.0; dim];
            for i in 0..dim {
                for (&ata_ij, &vj) in ata[i].iter().zip(v.iter()) {
                    av[i] += ata_ij * vj;
                }
            }
            let num: f64 = v.iter().zip(av.iter()).map(|(a, b)| a * b).sum();
            let den: f64 = v.iter().map(|x| x * x).sum();
            num / den.max(1e-30)
        };
        for (i, si) in shifted.iter_mut().enumerate() {
            si[i] -= lambda_est;
        }
        let w = gauss_solve_aug(&shifted, &v);
        let norm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-30 {
            break;
        }
        v = w.iter().map(|x| x / norm).collect();
    }
    let scale = if v[n].abs() > 1e-30 { -1.0 / v[n] } else { 0.0 };
    (0..n).map(|i| v[i] * scale).collect()
}

fn gauss_solve_aug(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut aug = vec![vec![0.0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = a[i][j];
        }
        aug[i][n] = b[i];
    }
    for col in 0..n {
        let mut max_row = col;
        for row in (col + 1)..n {
            if aug[row][col].abs() > aug[max_row][col].abs() {
                max_row = row;
            }
        }
        aug.swap(col, max_row);
        if aug[col][col].abs() < 1e-30 {
            continue;
        }
        for row in (col + 1)..n {
            let f = aug[row][col] / aug[col][col];
            let (top, bot) = aug.split_at_mut(row);
            for (d, &s) in bot[0][col..=n].iter_mut().zip(&top[col][col..=n]) {
                *d -= f * s;
            }
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = 0.0;
        for (&aij, &xj) in aug[i][(i + 1)..n].iter().zip(&x[(i + 1)..]) {
            s += aij * xj;
        }
        x[i] = if aug[i][i].abs() > 1e-30 {
            (aug[i][n] - s) / aug[i][i]
        } else {
            0.0
        };
    }
    x
}

pub fn iteratively_reweighted_least_squares(
    a: &[Vec<f64>],
    b: &[f64],
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let m = a.len();
    let mut w = vec![1.0; m];
    let mut x = normal_equations(a, b);
    for _ in 0..max_iter {
        let r = residual_vec(a, &x, b);
        let mut max_change = 0.0;
        for i in 0..m {
            let new_w = 1.0 / r[i].abs().max(1e-10);
            let change = (new_w - w[i]).abs();
            if change > max_change {
                max_change = change;
            }
            w[i] = new_w;
        }
        x = weighted_least_squares(a, b, &w);
        if max_change < tol {
            break;
        }
    }
    x
}

fn residual_vec(a: &[Vec<f64>], x: &[f64], b: &[f64]) -> Vec<f64> {
    let m = a.len();
    (0..m)
        .map(|i| {
            let mut ax = 0.0;
            for (&aij, &xj) in a[i].iter().zip(x.iter()) {
                ax += aij * xj;
            }
            b[i] - ax
        })
        .collect()
}

pub fn lasso_coordinate_descent(
    a: &[Vec<f64>],
    b: &[f64],
    lambda: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    let mut x = vec![0.0; n];
    for _ in 0..max_iter {
        let mut max_change = 0.0f64;
        for j in 0..n {
            let mut rj = 0.0;
            let mut aj_sq = 0.0;
            for i in 0..m {
                let mut pred = 0.0;
                for (k, (&aik, &xk)) in a[i].iter().zip(x.iter()).enumerate() {
                    if k != j {
                        pred += aik * xk;
                    }
                }
                rj += a[i][j] * (b[i] - pred);
                aj_sq += a[i][j] * a[i][j];
            }
            let new_x = if aj_sq.abs() < 1e-30 {
                0.0
            } else {
                soft_threshold(rj, lambda) / aj_sq
            };
            let change = (new_x - x[j]).abs();
            if change > max_change {
                max_change = change;
            }
            x[j] = new_x;
        }
        if max_change < tol {
            break;
        }
    }
    x
}

fn soft_threshold(val: f64, lambda: f64) -> f64 {
    if val > lambda {
        val - lambda
    } else if val < -lambda {
        val + lambda
    } else {
        0.0
    }
}

pub fn elastic_net(
    a: &[Vec<f64>],
    b: &[f64],
    lambda1: f64,
    lambda2: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    let mut x = vec![0.0; n];
    for _ in 0..max_iter {
        let mut max_change = 0.0f64;
        for j in 0..n {
            let mut rj = 0.0;
            let mut aj_sq = 0.0;
            for i in 0..m {
                let mut pred = 0.0;
                for (k, (&aik, &xk)) in a[i].iter().zip(x.iter()).enumerate() {
                    if k != j {
                        pred += aik * xk;
                    }
                }
                rj += a[i][j] * (b[i] - pred);
                aj_sq += a[i][j] * a[i][j];
            }
            let denom = aj_sq + lambda2;
            let new_x = if denom.abs() < 1e-30 {
                0.0
            } else {
                soft_threshold(rj, lambda1) / denom
            };
            let change = (new_x - x[j]).abs();
            if change > max_change {
                max_change = change;
            }
            x[j] = new_x;
        }
        if max_change < tol {
            break;
        }
    }
    x
}

pub fn cross_validation_score(x: &[f64], y: &[f64], degree: usize, folds: usize) -> f64 {
    let m = x.len();
    let fold_size = m / folds.max(1);
    let mut total_error = 0.0;
    for fold in 0..folds {
        let start = fold * fold_size;
        let end = if fold == folds - 1 {
            m
        } else {
            start + fold_size
        };
        let mut train_x = Vec::new();
        let mut train_y = Vec::new();
        let mut test_x = Vec::new();
        let mut test_y = Vec::new();
        for i in 0..m {
            if i >= start && i < end {
                test_x.push(x[i]);
                test_y.push(y[i]);
            } else {
                train_x.push(x[i]);
                train_y.push(y[i]);
            }
        }
        let coeffs = polynomial_fit(&train_x, &train_y, degree);
        let mut fold_error = 0.0;
        for i in 0..test_x.len() {
            let mut pred = 0.0;
            for (j, &c) in coeffs.iter().enumerate() {
                pred += c * test_x[i].powi(j as i32);
            }
            fold_error += (test_y[i] - pred).powi(2);
        }
        total_error += fold_error / test_x.len() as f64;
    }
    total_error / folds as f64
}
