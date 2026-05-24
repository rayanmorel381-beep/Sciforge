use super::linalg::matmul;
use super::ops::transpose;
use super::storage::Tensor;

pub fn qr_decompose(a: &Tensor) -> (Tensor, Tensor) {
    assert!(a.rank() == 2);
    let (m, n) = (a.shape()[0], a.shape()[1]);
    let mut q_cols: Vec<Vec<f64>> = Vec::new();

    for j in 0..n {
        let mut v: Vec<f64> = (0..m).map(|i| a.get(&[i, j])).collect();

        for q in &q_cols {
            let dot: f64 = v.iter().zip(q).map(|(a, b)| a * b).sum();
            for i in 0..m {
                v[i] -= dot * q[i];
            }
        }

        let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 1e-15 {
            for x in &mut v {
                *x /= norm;
            }
        }
        q_cols.push(v);
    }

    let q = Tensor::from_fn(&[m, n], |idx| q_cols[idx[1]][idx[0]]);
    let qt = transpose(&q, &[1, 0]);
    let r = matmul(&qt, a);
    (q, r)
}

pub fn cholesky(a: &Tensor) -> Option<Tensor> {
    assert!(a.rank() == 2 && a.shape()[0] == a.shape()[1]);
    let n = a.shape()[0];
    let mut l = Tensor::zeros(&[n, n]);

    for i in 0..n {
        for j in 0..=i {
            let mut sum = 0.0;
            for k in 0..j {
                sum += l.get(&[i, k]) * l.get(&[j, k]);
            }
            if i == j {
                let val = a.get(&[i, i]) - sum;
                if val <= 0.0 {
                    return None;
                }
                l.set(&[i, j], val.sqrt());
            } else {
                let ljj = l.get(&[j, j]);
                if ljj.abs() < 1e-30 {
                    return None;
                }
                l.set(&[i, j], (a.get(&[i, j]) - sum) / ljj);
            }
        }
    }
    Some(l)
}

pub fn eigenvalues_qr(a: &Tensor, max_iter: usize, tol: f64) -> Vec<f64> {
    assert!(a.rank() == 2 && a.shape()[0] == a.shape()[1]);
    let n = a.shape()[0];
    let mut ak = a.clone();

    for _ in 0..max_iter {
        let (q, r) = qr_decompose(&ak);
        ak = matmul(&r, &q);

        let mut off_diag = 0.0;
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    off_diag += ak.get(&[i, j]).powi(2);
                }
            }
        }
        if off_diag.sqrt() < tol {
            break;
        }
    }
    (0..n).map(|i| ak.get(&[i, i])).collect()
}

pub fn eigenvectors_qr(a: &Tensor, max_iter: usize, tol: f64) -> (Vec<f64>, Tensor) {
    assert!(a.rank() == 2 && a.shape()[0] == a.shape()[1]);
    let n = a.shape()[0];
    let mut ak = a.clone();
    let mut v = Tensor::identity(n);

    for _ in 0..max_iter {
        let (q, r) = qr_decompose(&ak);
        ak = matmul(&r, &q);
        v = matmul(&v, &q);

        let mut off_diag = 0.0;
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    off_diag += ak.get(&[i, j]).powi(2);
                }
            }
        }
        if off_diag.sqrt() < tol {
            break;
        }
    }
    let eigenvalues = (0..n).map(|i| ak.get(&[i, i])).collect();
    (eigenvalues, v)
}

pub fn svd(a: &Tensor) -> (Tensor, Vec<f64>, Tensor) {
    assert!(a.rank() == 2);
    let at = transpose(a, &[1, 0]);
    let ata = matmul(&at, a);
    let aat = matmul(a, &at);

    let (sigma_sq, v) = eigenvectors_qr(&ata, 200, 1e-12);
    let (_, u_raw) = eigenvectors_qr(&aat, 200, 1e-12);
    let singular_values: Vec<f64> = sigma_sq.iter().map(|&s| s.abs().sqrt()).collect();

    let m = a.shape()[0];
    let n = a.shape()[1];
    let k = m.min(n);

    let mut u_cols: Vec<Vec<f64>> = Vec::new();
    for (j, &sv) in singular_values[..k].iter().enumerate() {
        if sv > 1e-15 {
            let uj: Vec<f64> = (0..m).map(|i| u_raw.get(&[i, j])).collect();
            let vj: Vec<f64> = (0..n).map(|i| v.get(&[i, j])).collect();
            let vj_tensor = Tensor::from_vec(&[n, 1], vj);
            let av = matmul(a, &vj_tensor);
            let dot: f64 = av.data().iter().zip(&uj).map(|(a, b)| a * b).sum();
            let sign = if dot >= 0.0 { 1.0 } else { -1.0 };
            u_cols.push(uj.iter().map(|&x| x * sign).collect());
        } else {
            u_cols.push(vec![0.0; m]);
        }
    }

    let u = Tensor::from_fn(&[m, k], |idx| u_cols[idx[1]][idx[0]]);
    (u, singular_values[..k].to_vec(), v)
}

pub fn pseudoinverse(a: &Tensor) -> Tensor {
    let (u, sigma, v) = svd(a);
    let m = a.shape()[0];
    let n = a.shape()[1];
    let k = sigma.len();

    let sigma_inv = Tensor::from_fn(&[k, k], |idx| {
        if idx[0] == idx[1] && sigma[idx[0]].abs() > 1e-12 {
            1.0 / sigma[idx[0]]
        } else {
            0.0
        }
    });

    let ut = transpose(&u, &[1, 0]);
    let tmp = matmul(&sigma_inv, &ut);
    let result = matmul(&v, &tmp);
    assert!(result.shape()[0] == n && result.shape()[1] == m);
    result
}

pub fn condition_number(a: &Tensor) -> f64 {
    let (_, sigma, _) = svd(a);
    let max_s = sigma.iter().cloned().fold(0.0_f64, f64::max);
    let min_s = sigma
        .iter()
        .cloned()
        .filter(|&s| s > 1e-15)
        .fold(f64::INFINITY, f64::min);
    if min_s == f64::INFINITY {
        return f64::INFINITY;
    }
    max_s / min_s
}

pub fn matrix_exp(a: &Tensor, terms: usize) -> Tensor {
    assert!(a.rank() == 2 && a.shape()[0] == a.shape()[1]);
    let n = a.shape()[0];
    let mut result = Tensor::identity(n);
    let mut power = Tensor::identity(n);
    let mut factorial = 1.0;

    for k in 1..terms {
        power = matmul(&power, a);
        factorial *= k as f64;
        result = &result + &power.scale(1.0 / factorial);
    }
    result
}

pub fn matrix_norm_1(a: &Tensor) -> f64 {
    assert!(a.rank() == 2);
    let (m, n) = (a.shape()[0], a.shape()[1]);
    let mut max_col = 0.0_f64;
    for j in 0..n {
        let col_sum: f64 = (0..m).map(|i| a.get(&[i, j]).abs()).sum();
        max_col = max_col.max(col_sum);
    }
    max_col
}

pub fn matrix_norm_inf(a: &Tensor) -> f64 {
    assert!(a.rank() == 2);
    let (m, n) = (a.shape()[0], a.shape()[1]);
    let mut max_row = 0.0_f64;
    for i in 0..m {
        let row_sum: f64 = (0..n).map(|j| a.get(&[i, j]).abs()).sum();
        max_row = max_row.max(row_sum);
    }
    max_row
}

pub fn power_iteration(a: &Tensor, max_iter: usize, tol: f64) -> (f64, Vec<f64>) {
    assert!(a.rank() == 2 && a.shape()[0] == a.shape()[1]);
    let n = a.shape()[0];
    let mut v = vec![1.0; n];
    let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    for x in &mut v {
        *x /= norm;
    }

    let mut eigenvalue = 0.0;
    for _ in 0..max_iter {
        let vt = Tensor::from_vec(&[n, 1], v.clone());
        let av = matmul(a, &vt);
        let new_eigenvalue = av.data().iter().zip(&v).map(|(a, b)| a * b).sum::<f64>();
        let norm: f64 = av.data().iter().map(|x| x * x).sum::<f64>().sqrt();
        v = av.data().iter().map(|x| x / norm).collect();
        if (new_eigenvalue - eigenvalue).abs() < tol {
            break;
        }
        eigenvalue = new_eigenvalue;
    }
    (eigenvalue, v)
}
