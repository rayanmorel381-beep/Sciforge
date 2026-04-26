use super::csr::SparseMatrix;

pub fn conjugate_gradient(a: &SparseMatrix, b: &[f64], tol: f64, max_iter: usize) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];
    let mut r: Vec<f64> = b.to_vec();
    let mut p = r.clone();
    let mut rs_old: f64 = r.iter().map(|v| v * v).sum();

    for _ in 0..max_iter {
        if rs_old.sqrt() < tol {
            break;
        }
        let ap = a.mul_vec(&p);
        let pap: f64 = p.iter().zip(&ap).map(|(a, b)| a * b).sum();
        if pap.abs() < 1e-30 {
            break;
        }
        let alpha = rs_old / pap;

        for (i, xi) in x.iter_mut().enumerate() {
            *xi += alpha * p[i];
            r[i] -= alpha * ap[i];
        }

        let rs_new: f64 = r.iter().map(|v| v * v).sum();
        if rs_new.sqrt() < tol {
            break;
        }
        let beta = rs_new / rs_old;
        for (i, pi) in p.iter_mut().enumerate() {
            *pi = r[i] + beta * *pi;
        }
        rs_old = rs_new;
    }
    x
}

pub fn jacobi_iterate(a: &SparseMatrix, b: &[f64], tol: f64, max_iter: usize) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];

    for _ in 0..max_iter {
        let mut x_new = vec![0.0; n];
        for i in 0..n {
            let aii = a.get(i, i);
            if aii.abs() < 1e-30 {
                continue;
            }
            let ax: f64 = (a.row_ptr[i]..a.row_ptr[i + 1])
                .filter(|&k| a.col_idx[k] != i)
                .map(|k| a.values[k] * x[a.col_idx[k]])
                .sum();
            x_new[i] = (b[i] - ax) / aii;
        }
        let err: f64 = x_new
            .iter()
            .zip(&x)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();
        x = x_new;
        if err < tol {
            break;
        }
    }
    x
}

pub fn gauss_seidel(a: &SparseMatrix, b: &[f64], tol: f64, max_iter: usize) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];

    for _ in 0..max_iter {
        let x_old = x.clone();
        for i in 0..n {
            let aii = a.get(i, i);
            if aii.abs() < 1e-30 {
                continue;
            }
            let ax: f64 = (a.row_ptr[i]..a.row_ptr[i + 1])
                .filter(|&k| a.col_idx[k] != i)
                .map(|k| a.values[k] * x[a.col_idx[k]])
                .sum();
            x[i] = (b[i] - ax) / aii;
        }
        let err: f64 = x
            .iter()
            .zip(&x_old)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();
        if err < tol {
            break;
        }
    }
    x
}

pub fn sparse_lu_solve(a: &SparseMatrix, b: &[f64]) -> Vec<f64> {
    let dense = a.to_dense();
    let n = b.len();
    let mut l = vec![vec![0.0; n]; n];
    let mut u = dense.clone();

    for (i, li) in l.iter_mut().enumerate() {
        li[i] = 1.0;
    }

    for j in 0..n {
        for i in j + 1..n {
            if u[j][j].abs() < 1e-30 {
                continue;
            }
            let factor = u[i][j] / u[j][j];
            l[i][j] = factor;
            let (top, bot) = u.split_at_mut(i);
            for (d, &s) in bot[0][j..n].iter_mut().zip(&top[j][j..n]) {
                *d -= factor * s;
            }
        }
    }

    let mut y = vec![0.0; n];
    for i in 0..n {
        y[i] = b[i];
        for j in 0..i {
            y[i] -= l[i][j] * y[j];
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = y[i];
        for j in i + 1..n {
            x[i] -= u[i][j] * x[j];
        }
        if u[i][i].abs() > 1e-30 {
            x[i] /= u[i][i];
        }
    }
    x
}

pub fn sor_iterate(a: &SparseMatrix, b: &[f64], omega: f64, tol: f64, max_iter: usize) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];
    for _ in 0..max_iter {
        let x_old = x.clone();
        for i in 0..n {
            let aii = a.get(i, i);
            if aii.abs() < 1e-30 {
                continue;
            }
            let ax: f64 = (a.row_ptr[i]..a.row_ptr[i + 1])
                .filter(|&k| a.col_idx[k] != i)
                .map(|k| a.values[k] * x[a.col_idx[k]])
                .sum();
            let gs_val = (b[i] - ax) / aii;
            x[i] = (1.0 - omega) * x_old[i] + omega * gs_val;
        }
        let err: f64 = x
            .iter()
            .zip(&x_old)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();
        if err < tol {
            break;
        }
    }
    x
}

pub fn steepest_descent(a: &SparseMatrix, b: &[f64], tol: f64, max_iter: usize) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];
    for _ in 0..max_iter {
        let ax = a.mul_vec(&x);
        let r: Vec<f64> = (0..n).map(|i| b[i] - ax[i]).collect();
        let rnorm: f64 = r.iter().map(|v| v * v).sum::<f64>().sqrt();
        if rnorm < tol {
            break;
        }
        let ar = a.mul_vec(&r);
        let rar: f64 = r.iter().zip(&ar).map(|(ri, ari)| ri * ari).sum();
        if rar.abs() < 1e-30 {
            break;
        }
        let alpha = rnorm * rnorm / rar;
        for (xi, &ri) in x.iter_mut().zip(r.iter()) {
            *xi += alpha * ri;
        }
    }
    x
}

pub fn bicgstab(a: &SparseMatrix, b: &[f64], tol: f64, max_iter: usize) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];
    let mut r: Vec<f64> = b.to_vec();
    let r0 = r.clone();
    let mut p = r.clone();
    let mut rho: f64 = r0.iter().zip(&r).map(|(a, b)| a * b).sum();

    for _ in 0..max_iter {
        let ap = a.mul_vec(&p);
        let r0ap: f64 = r0.iter().zip(&ap).map(|(a, b)| a * b).sum();
        if r0ap.abs() < 1e-30 {
            break;
        }
        let alpha = rho / r0ap;
        let s: Vec<f64> = (0..n).map(|i| r[i] - alpha * ap[i]).collect();
        let snorm: f64 = s.iter().map(|v| v * v).sum::<f64>().sqrt();
        if snorm < tol {
            for (xi, &pi) in x.iter_mut().zip(p.iter()) {
                *xi += alpha * pi;
            }
            break;
        }
        let as_vec = a.mul_vec(&s);
        let as_s: f64 = as_vec.iter().zip(&s).map(|(a, b)| a * b).sum();
        let as_as: f64 = as_vec.iter().map(|v| v * v).sum();
        let omega = if as_as.abs() > 1e-30 {
            as_s / as_as
        } else {
            0.0
        };
        for (i, xi) in x.iter_mut().enumerate() {
            *xi += alpha * p[i] + omega * s[i];
            r[i] = s[i] - omega * as_vec[i];
        }
        let rnorm: f64 = r.iter().map(|v| v * v).sum::<f64>().sqrt();
        if rnorm < tol {
            break;
        }
        let rho_new: f64 = r0.iter().zip(&r).map(|(a, b)| a * b).sum();
        if rho.abs() < 1e-30 {
            break;
        }
        let beta = (rho_new / rho) * (alpha / omega);
        for (i, pi) in p.iter_mut().enumerate() {
            *pi = r[i] + beta * (*pi - omega * ap[i]);
        }
        rho = rho_new;
    }
    x
}

pub fn preconditioned_cg(
    a: &SparseMatrix,
    b: &[f64],
    precond_solve: impl Fn(&[f64]) -> Vec<f64>,
    tol: f64,
    max_iter: usize,
) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];
    let mut r: Vec<f64> = b.to_vec();
    let mut z = precond_solve(&r);
    let mut p = z.clone();
    let mut rz: f64 = r.iter().zip(&z).map(|(a, b)| a * b).sum();

    for _ in 0..max_iter {
        if rz.abs().sqrt() < tol {
            break;
        }
        let ap = a.mul_vec(&p);
        let pap: f64 = p.iter().zip(&ap).map(|(a, b)| a * b).sum();
        if pap.abs() < 1e-30 {
            break;
        }
        let alpha = rz / pap;
        for (i, xi) in x.iter_mut().enumerate() {
            *xi += alpha * p[i];
            r[i] -= alpha * ap[i];
        }
        let rnorm: f64 = r.iter().map(|v| v * v).sum::<f64>().sqrt();
        if rnorm < tol {
            break;
        }
        z = precond_solve(&r);
        let rz_new: f64 = r.iter().zip(&z).map(|(a, b)| a * b).sum();
        let beta = rz_new / rz;
        for (pi, &zi) in p.iter_mut().zip(z.iter()) {
            *pi = zi + beta * *pi;
        }
        rz = rz_new;
    }
    x
}

pub fn gmres(a: &SparseMatrix, b: &[f64], tol: f64, max_iter: usize, restart: usize) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];
    for _ in 0..max_iter / restart.max(1) {
        let ax = a.mul_vec(&x);
        let r: Vec<f64> = (0..n).map(|i| b[i] - ax[i]).collect();
        let beta = r.iter().map(|v| v * v).sum::<f64>().sqrt();
        if beta < tol {
            break;
        }
        let mut v = vec![vec![0.0; n]; restart + 1];
        for (v0i, &ri) in v[0].iter_mut().zip(r.iter()) {
            *v0i = ri / beta;
        }
        let mut h = vec![vec![0.0; restart]; restart + 1];
        let mut g = vec![0.0; restart + 1];
        g[0] = beta;
        let mut cs = vec![0.0; restart];
        let mut sn = vec![0.0; restart];
        let mut k = 0;
        for j in 0..restart {
            k = j + 1;
            let w = a.mul_vec(&v[j]);
            let mut wk = w;
            for i in 0..=j {
                h[i][j] = (0..n).map(|l| wk[l] * v[i][l]).sum();
                for (wk_l, &vil) in wk.iter_mut().zip(v[i].iter()) {
                    *wk_l -= h[i][j] * vil;
                }
            }
            h[j + 1][j] = wk.iter().map(|v| v * v).sum::<f64>().sqrt();
            if h[j + 1][j].abs() < 1e-30 {
                break;
            }
            v[j + 1] = wk.iter().map(|v| v / h[j + 1][j]).collect();
            for i in 0..j {
                let temp = cs[i] * h[i][j] + sn[i] * h[i + 1][j];
                h[i + 1][j] = -sn[i] * h[i][j] + cs[i] * h[i + 1][j];
                h[i][j] = temp;
            }
            let rr = (h[j][j] * h[j][j] + h[j + 1][j] * h[j + 1][j]).sqrt();
            cs[j] = h[j][j] / rr;
            sn[j] = h[j + 1][j] / rr;
            h[j][j] = rr;
            h[j + 1][j] = 0.0;
            g[j + 1] = -sn[j] * g[j];
            g[j] *= cs[j];
            if g[j + 1].abs() < tol {
                k = j + 1;
                break;
            }
        }
        let mut y = vec![0.0; k];
        for i in (0..k).rev() {
            y[i] = g[i];
            for j in i + 1..k {
                y[i] -= h[i][j] * y[j];
            }
            y[i] /= h[i][i];
        }
        for (i, xi) in x.iter_mut().enumerate() {
            for j in 0..k {
                *xi += y[j] * v[j][i];
            }
        }
        let ax2 = a.mul_vec(&x);
        let res: f64 = (0..n).map(|i| (b[i] - ax2[i]).powi(2)).sum::<f64>().sqrt();
        if res < tol {
            break;
        }
    }
    x
}
