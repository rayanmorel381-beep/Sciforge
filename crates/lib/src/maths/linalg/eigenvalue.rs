pub fn power_iteration(a: &[Vec<f64>], max_iter: usize, tol: f64) -> (f64, Vec<f64>) {
    let n = a.len();
    let mut v = vec![1.0 / (n as f64).sqrt(); n];
    let mut eigenvalue = 0.0;
    for _ in 0..max_iter {
        let mut av = vec![0.0; n];
        for i in 0..n {
            for (&aij, &vj) in a[i].iter().zip(v.iter()) {
                av[i] += aij * vj;
            }
        }
        let new_eigenvalue = av.iter().zip(v.iter()).map(|(&a, &b)| a * b).sum::<f64>();
        let norm = av.iter().map(|x| x * x).sum::<f64>().sqrt();
        for i in 0..n {
            v[i] = av[i] / norm;
        }
        if (new_eigenvalue - eigenvalue).abs() < tol {
            break;
        }
        eigenvalue = new_eigenvalue;
    }
    (eigenvalue, v)
}

pub fn rayleigh_quotient(a: &[Vec<f64>], v: &[f64]) -> f64 {
    let n = a.len();
    let mut av = vec![0.0; n];
    for i in 0..n {
        for (&aij, &vj) in a[i].iter().zip(v.iter()) {
            av[i] += aij * vj;
        }
    }
    let num: f64 = v.iter().zip(av.iter()).map(|(&a, &b)| a * b).sum();
    let den: f64 = v.iter().map(|x| x * x).sum();
    num / den
}

pub fn gershgorin_disks(a: &[Vec<f64>]) -> Vec<(f64, f64)> {
    let mut disks = Vec::with_capacity(a.len());
    for (i, ai) in a.iter().enumerate() {
        let center = ai[i];
        let mut radius = 0.0;
        for (j, &aij) in ai.iter().enumerate() {
            if j != i {
                radius += aij.abs();
            }
        }
        disks.push((center, radius));
    }
    disks
}

pub fn trace(a: &[Vec<f64>]) -> f64 {
    a.iter().enumerate().map(|(i, row)| row[i]).sum()
}

pub fn spectral_radius(eigenvalues: &[f64]) -> f64 {
    eigenvalues.iter().map(|e| e.abs()).fold(0.0_f64, f64::max)
}

pub fn condition_number(eigenvalues: &[f64]) -> f64 {
    let max = eigenvalues.iter().map(|e| e.abs()).fold(0.0_f64, f64::max);
    let min = eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(f64::INFINITY, f64::min);
    max / min.max(1e-30)
}

pub fn inverse_iteration(a: &[Vec<f64>], sigma: f64, max_iter: usize, tol: f64) -> (f64, Vec<f64>) {
    let n = a.len();
    let mut shifted = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            shifted[i][j] = a[i][j];
        }
        shifted[i][i] -= sigma;
    }
    let mut v = vec![1.0 / (n as f64).sqrt(); n];
    let mut eigenvalue = sigma;
    for _ in 0..max_iter {
        let w = solve_simple(&shifted, &v);
        let norm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
        let new_v: Vec<f64> = w.iter().map(|x| x / norm).collect();
        let new_eigenvalue = rayleigh_quotient(a, &new_v);
        if (new_eigenvalue - eigenvalue).abs() < tol {
            return (new_eigenvalue, new_v);
        }
        eigenvalue = new_eigenvalue;
        v = new_v;
    }
    (eigenvalue, v)
}

fn solve_simple(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
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
        for (&aij, &xj) in aug[i][(i + 1)..].iter().zip(&x[(i + 1)..]) {
            sum += aij * xj;
        }
        x[i] = (aug[i][n] - sum) / aug[i][i];
    }
    x
}

pub fn qr_algorithm(a: &[Vec<f64>], max_iter: usize, tol: f64) -> Vec<f64> {
    let n = a.len();
    let mut work = a.to_vec();
    for _ in 0..max_iter {
        let (q, r) = qr_decompose_internal(&work);
        let mut new = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                for (&rik, qk) in r[i].iter().zip(q.iter()) {
                    new[i][j] += rik * qk[j];
                }
            }
        }
        let mut off_diag = 0.0;
        for (i, new_i) in new.iter().enumerate() {
            for (j, &nij) in new_i.iter().enumerate() {
                if i != j {
                    off_diag += nij.abs();
                }
            }
        }
        work = new;
        if off_diag < tol {
            break;
        }
    }
    (0..n).map(|i| work[i][i]).collect()
}

fn qr_decompose_internal(a: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let n = a.len();
    let mut q = vec![vec![0.0; n]; n];
    let mut r = vec![vec![0.0; n]; n];
    let cols: Vec<Vec<f64>> = (0..n).map(|j| (0..n).map(|i| a[i][j]).collect()).collect();
    let mut u: Vec<Vec<f64>> = Vec::with_capacity(n);
    for j in 0..n {
        let mut v = cols[j].clone();
        for i in 0..j {
            let dot: f64 = (0..n).map(|k| u[i][k] * cols[j][k]).sum();
            r[i][j] = dot;
            for (vk, &uik) in v.iter_mut().zip(u[i].iter()) {
                *vk -= dot * uik;
            }
        }
        let norm = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        r[j][j] = norm;
        if norm > 1e-30 {
            for vk in v.iter_mut() {
                *vk /= norm;
            }
        }
        u.push(v);
    }
    for (i, qi) in q.iter_mut().enumerate() {
        for (qij, uj) in qi.iter_mut().zip(u.iter()) {
            *qij = uj[i];
        }
    }
    (q, r)
}

pub fn eigenvalues_symmetric_tridiag(
    diag: &[f64],
    off_diag: &[f64],
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let n = diag.len();
    let mut d = diag.to_vec();
    let mut e = off_diag.to_vec();
    e.push(0.0);
    for _ in 0..max_iter {
        let mut converged = true;
        for ei in &e[..(n - 1)] {
            if ei.abs() > tol {
                converged = false;
                break;
            }
        }
        if converged {
            break;
        }
        for l in 0..n {
            if l >= n - 1 {
                break;
            }
            if e[l].abs() < tol {
                continue;
            }
            let shift = d[n - 1];
            for di in d.iter_mut() {
                *di -= shift;
            }
            let mut x = d[0];
            let mut z = e[0];
            for i in 0..(n - 1) {
                let r = (x * x + z * z).sqrt();
                let c = x / r.max(1e-30);
                let s = z / r.max(1e-30);
                if i > 0 {
                    e[i - 1] = r;
                }
                x = c * d[i] + s * e[i];
                let t = -s * d[i] + c * e[i];
                d[i] = c * x + s * t;
                x = -s * x + c * t;
                if i + 1 < n {
                    z = s * e[i + 1];
                    e[i + 1] *= c;
                }
                let _ = t;
            }
            d[n - 1] = x;
            e[n - 2] = 0.0;
            for di in d.iter_mut() {
                *di += shift;
            }
            break;
        }
    }
    d
}

pub fn deflated_power_iteration(
    a: &[Vec<f64>],
    num_eigenvalues: usize,
    max_iter: usize,
    tol: f64,
) -> Vec<(f64, Vec<f64>)> {
    let n = a.len();
    let mut work = a.to_vec();
    let mut results = Vec::new();
    for _ in 0..num_eigenvalues.min(n) {
        let (eigenvalue, eigenvector) = power_iteration(&work, max_iter, tol);
        results.push((eigenvalue, eigenvector.clone()));
        for i in 0..n {
            for j in 0..n {
                work[i][j] -= eigenvalue * eigenvector[i] * eigenvector[j];
            }
        }
    }
    results
}

pub fn is_eigenvalue(a: &[Vec<f64>], lambda: f64, tol: f64) -> bool {
    let mut shifted = a.to_vec();
    for (i, si) in shifted.iter_mut().enumerate() {
        si[i] -= lambda;
    }
    det_abs(&shifted) < tol
}

fn det_abs(a: &[Vec<f64>]) -> f64 {
    let n = a.len();
    let mut work = a.to_vec();
    let mut sign = 1.0;
    for col in 0..n {
        let mut pivot = col;
        for row in (col + 1)..n {
            if work[row][col].abs() > work[pivot][col].abs() {
                pivot = row;
            }
        }
        if pivot != col {
            work.swap(col, pivot);
            sign = -sign;
        }
        if work[col][col].abs() < 1e-30 {
            return 0.0;
        }
        for row in (col + 1)..n {
            let f = work[row][col] / work[col][col];
            let (top, bot) = work.split_at_mut(row);
            for (d, &s) in bot[0][col..n].iter_mut().zip(&top[col][col..n]) {
                *d -= f * s;
            }
        }
    }
    let mut d = sign;
    for (i, wi) in work.iter().enumerate() {
        d *= wi[i];
    }
    d.abs()
}

pub fn characteristic_polynomial_coeffs(a: &[Vec<f64>]) -> Vec<f64> {
    let n = a.len();
    let eigenvalues = qr_algorithm(a, 200, 1e-10);
    let mut coeffs = vec![1.0];
    for &ev in &eigenvalues {
        let m = coeffs.len();
        let mut new_coeffs = vec![0.0; m + 1];
        for i in 0..m {
            new_coeffs[i + 1] += coeffs[i];
            new_coeffs[i] -= ev * coeffs[i];
        }
        coeffs = new_coeffs;
    }
    let _ = n;
    coeffs
}

pub fn matrix_exponential_diag(eigenvalues: &[f64], eigenvectors: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = eigenvalues.len();
    let mut result = vec![vec![0.0; n]; n];
    for (k, &ev_k) in eigenvalues.iter().enumerate() {
        let exp_k = ev_k.exp();
        for i in 0..n {
            for j in 0..n {
                result[i][j] += exp_k * eigenvectors[i][k] * eigenvectors[j][k];
            }
        }
    }
    result
}
