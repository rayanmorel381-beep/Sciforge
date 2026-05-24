pub fn lu_decompose(a: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    let mut u = vec![vec![0.0; n]; n];
    for i in 0..n {
        for k in i..n {
            let mut sum = 0.0;
            for (&lij, uj) in l[i][..i].iter().zip(&u[..i]) {
                sum += lij * uj[k];
            }
            u[i][k] = a[i][k] - sum;
        }
        for k in i..n {
            if i == k {
                l[i][i] = 1.0;
            } else {
                let mut sum = 0.0;
                for (&lkj, uj) in l[k][..i].iter().zip(&u[..i]) {
                    sum += lkj * uj[i];
                }
                l[k][i] = (a[k][i] - sum) / u[i][i];
            }
        }
    }
    (l, u)
}

pub fn forward_substitution(l: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut sum = 0.0;
        for (&lij, &yj) in l[i][..i].iter().zip(&y[..i]) {
            sum += lij * yj;
        }
        y[i] = (b[i] - sum) / l[i][i];
    }
    y
}

pub fn back_substitution(u: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let n = y.len();
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = 0.0;
        for (&uij, &xj) in u[i][(i + 1)..].iter().zip(&x[(i + 1)..]) {
            sum += uij * xj;
        }
        x[i] = (y[i] - sum) / u[i][i];
    }
    x
}

pub fn lu_solve(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let (l, u) = lu_decompose(a);
    let y = forward_substitution(&l, b);
    back_substitution(&u, &y)
}

pub fn cholesky(a: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut sum = 0.0;
            for (&a, &b) in l[i][..j].iter().zip(&l[j][..j]) {
                sum += a * b;
            }
            if i == j {
                l[i][j] = (a[i][i] - sum).sqrt();
            } else {
                l[i][j] = (a[i][j] - sum) / l[j][j];
            }
        }
    }
    l
}

pub fn determinant_lu(a: &[Vec<f64>]) -> f64 {
    let (_, u) = lu_decompose(a);
    let mut det = 1.0;
    for (i, ui) in u.iter().enumerate() {
        det *= ui[i];
    }
    det
}

pub fn matrix_inverse_lu(a: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let (l, u) = lu_decompose(a);
    let mut inv = vec![vec![0.0; n]; n];
    for col in 0..n {
        let mut e = vec![0.0; n];
        e[col] = 1.0;
        let y = forward_substitution(&l, &e);
        let x = back_substitution(&u, &y);
        for row in 0..n {
            inv[row][col] = x[row];
        }
    }
    inv
}

pub fn lu_decompose_partial_pivot(a: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<usize>) {
    let n = a.len();
    let mut work = a.to_vec();
    let mut l = vec![vec![0.0; n]; n];
    let mut perm: Vec<usize> = (0..n).collect();
    for k in 0..n {
        let mut max_row = k;
        for i in (k + 1)..n {
            if work[i][k].abs() > work[max_row][k].abs() {
                max_row = i;
            }
        }
        work.swap(k, max_row);
        l.swap(k, max_row);
        perm.swap(k, max_row);
        l[k][k] = 1.0;
        for i in (k + 1)..n {
            if work[k][k].abs() < 1e-30 {
                continue;
            }
            l[i][k] = work[i][k] / work[k][k];
            let lik = l[i][k];
            let (top, bot) = work.split_at_mut(i);
            for (d, &s) in bot[0][k..n].iter_mut().zip(&top[k][k..n]) {
                *d -= lik * s;
            }
        }
    }
    (l, work, perm)
}

pub fn ldl_decompose(a: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<f64>) {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    let mut d = vec![0.0; n];
    for i in 0..n {
        l[i][i] = 1.0;
        let mut sum = 0.0;
        for (&lik, &dk) in l[i][..i].iter().zip(&d[..i]) {
            sum += lik * lik * dk;
        }
        d[i] = a[i][i] - sum;
        for j in (i + 1)..n {
            let mut s = 0.0;
            for ((&ljk, &dk), &lik) in l[j][..i].iter().zip(&d[..i]).zip(&l[i][..i]) {
                s += ljk * dk * lik;
            }
            l[j][i] = if d[i].abs() > 1e-30 {
                (a[j][i] - s) / d[i]
            } else {
                0.0
            };
        }
    }
    (l, d)
}

pub fn cholesky_solve(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let l = cholesky(a);
    let n = b.len();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut s = 0.0;
        for (&lij, &yj) in l[i][..i].iter().zip(&y[..i]) {
            s += lij * yj;
        }
        y[i] = (b[i] - s) / l[i][i];
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = 0.0;
        for j in (i + 1)..n {
            s += l[j][i] * x[j];
        }
        x[i] = (y[i] - s) / l[i][i];
    }
    x
}

pub fn crout_decompose(a: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    let mut u = vec![vec![0.0; n]; n];
    for (j, uj) in u.iter_mut().enumerate() {
        uj[j] = 1.0;
    }
    for j in 0..n {
        for i in j..n {
            let mut s = 0.0;
            for (&lik, uk) in l[i][..j].iter().zip(&u[..j]) {
                s += lik * uk[j];
            }
            l[i][j] = a[i][j] - s;
        }
        for i in (j + 1)..n {
            let mut s = 0.0;
            for (&ljk, uk) in l[j][..j].iter().zip(&u[..j]) {
                s += ljk * uk[i];
            }
            u[j][i] = if l[j][j].abs() > 1e-30 {
                (a[j][i] - s) / l[j][j]
            } else {
                0.0
            };
        }
    }
    (l, u)
}

pub fn determinant_cholesky(a: &[Vec<f64>]) -> f64 {
    let l = cholesky(a);
    let mut det = 1.0;
    for (i, li) in l.iter().enumerate() {
        det *= li[i] * li[i];
    }
    det
}

pub fn schur_complement(m: &[Vec<f64>], k: usize) -> Vec<Vec<f64>> {
    let n = m.len();
    let p = n - k;
    let mut a = vec![vec![0.0; k]; k];
    let mut b = vec![vec![0.0; p]; k];
    let mut c = vec![vec![0.0; k]; p];
    let mut d = vec![vec![0.0; p]; p];
    for i in 0..k {
        for j in 0..k {
            a[i][j] = m[i][j];
        }
    }
    for i in 0..k {
        for j in 0..p {
            b[i][j] = m[i][k + j];
        }
    }
    for i in 0..p {
        for j in 0..k {
            c[i][j] = m[k + i][j];
        }
    }
    for i in 0..p {
        for j in 0..p {
            d[i][j] = m[k + i][k + j];
        }
    }
    let a_inv = matrix_inverse_lu(&a);
    let mut ca_inv = vec![vec![0.0; k]; p];
    for i in 0..p {
        for j in 0..k {
            for (&cil, a_inv_l) in c[i][..k].iter().zip(&a_inv[..k]) {
                ca_inv[i][j] += cil * a_inv_l[j];
            }
        }
    }
    let mut result = vec![vec![0.0; p]; p];
    for i in 0..p {
        for j in 0..p {
            result[i][j] = d[i][j];
            for (&ca_il, bl) in ca_inv[i][..k].iter().zip(&b[..k]) {
                result[i][j] -= ca_il * bl[j];
            }
        }
    }
    result
}

pub fn solve_triangular_lower(l: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    forward_substitution(l, b)
}

pub fn solve_triangular_upper(u: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    back_substitution(u, b)
}
