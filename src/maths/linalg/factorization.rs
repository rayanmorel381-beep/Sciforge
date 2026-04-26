pub fn qr_gram_schmidt(a: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    let mut q = vec![vec![0.0; m]; n];
    let mut r = vec![vec![0.0; n]; n];
    for j in 0..n {
        let mut v: Vec<f64> = (0..m).map(|i| a[i][j]).collect();
        for i in 0..j {
            let dot: f64 = (0..m).map(|k| q[i][k] * v[k]).sum();
            r[i][j] = dot;
            for (vk, &qik) in v.iter_mut().zip(q[i].iter()) {
                *vk -= dot * qik;
            }
        }
        let norm = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        r[j][j] = norm;
        if norm > 1e-30 {
            for (qjk, &vk) in q[j].iter_mut().zip(v.iter()) {
                *qjk = vk / norm;
            }
        }
    }
    let q_matrix = (0..m).map(|i| (0..n).map(|j| q[j][i]).collect()).collect();
    (q_matrix, r)
}

pub fn qr_solve(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let (q, r) = qr_gram_schmidt(a);
    let m = q.len();
    let n = r.len();
    let mut qtb = vec![0.0; n];
    for (j, qtb_j) in qtb.iter_mut().enumerate() {
        for i in 0..m {
            *qtb_j += q[i][j] * b[i];
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = 0.0;
        for (&rij, &xj) in r[i][(i + 1)..].iter().zip(&x[(i + 1)..]) {
            sum += rij * xj;
        }
        x[i] = (qtb[i] - sum) / r[i][i];
    }
    x
}

pub fn householder_reflection(v: &[f64]) -> Vec<Vec<f64>> {
    let n = v.len();
    let dot: f64 = v.iter().map(|x| x * x).sum();
    let mut h = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            h[i][j] = if i == j { 1.0 } else { 0.0 } - 2.0 * v[i] * v[j] / dot.max(1e-30);
        }
    }
    h
}

pub fn rank(a: &[Vec<f64>], tol: f64) -> usize {
    let m = a.len();
    let n = if m > 0 {
        a[0].len()
    } else {
        return 0;
    };
    let mut work: Vec<Vec<f64>> = a.to_vec();
    let mut r = 0;
    for col in 0..n {
        let mut pivot = r;
        for row in (r + 1)..m {
            if work[row][col].abs() > work[pivot][col].abs() {
                pivot = row;
            }
        }
        if work[pivot][col].abs() < tol {
            continue;
        }
        work.swap(r, pivot);
        let div = work[r][col];
        work[r][col..n].iter_mut().for_each(|v| *v /= div);
        for row in 0..m {
            if row == r {
                continue;
            }
            let factor = work[row][col];
            let src = work[r][col..n].to_vec();
            work[row][col..n]
                .iter_mut()
                .zip(&src)
                .for_each(|(d, &s)| *d -= factor * s);
        }
        r += 1;
    }
    r
}

pub fn null_space_dimension(a: &[Vec<f64>], tol: f64) -> usize {
    let n = if a.is_empty() { 0 } else { a[0].len() };
    n - rank(a, tol)
}

pub fn matrix_norm_frobenius(a: &[Vec<f64>]) -> f64 {
    a.iter()
        .flat_map(|row| row.iter())
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt()
}

pub fn matrix_norm_inf(a: &[Vec<f64>]) -> f64 {
    a.iter()
        .map(|row| row.iter().map(|x| x.abs()).sum::<f64>())
        .fold(0.0_f64, f64::max)
}

pub fn is_symmetric(a: &[Vec<f64>], tol: f64) -> bool {
    for (i, ai) in a.iter().enumerate() {
        for (j, aj) in a.iter().enumerate().skip(i + 1) {
            if (ai[j] - aj[i]).abs() > tol {
                return false;
            }
        }
    }
    true
}

pub fn is_positive_definite(a: &[Vec<f64>]) -> bool {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut sum = 0.0;
            for (&a, &b) in l[i][..j].iter().zip(&l[j][..j]) {
                sum += a * b;
            }
            if i == j {
                let val = a[i][i] - sum;
                if val <= 0.0 {
                    return false;
                }
                l[i][j] = val.sqrt();
            } else {
                l[i][j] = (a[i][j] - sum) / l[j][j];
            }
        }
    }
    true
}

pub fn is_orthogonal(a: &[Vec<f64>], tol: f64) -> bool {
    let n = a.len();
    for i in 0..n {
        for j in 0..n {
            let mut dot = 0.0;
            for ak in a.iter() {
                dot += ak[i] * ak[j];
            }
            let expected = if i == j { 1.0 } else { 0.0 };
            if (dot - expected).abs() > tol {
                return false;
            }
        }
    }
    true
}

pub fn is_diagonal(a: &[Vec<f64>], tol: f64) -> bool {
    for (i, ai) in a.iter().enumerate() {
        for (j, &aij) in ai.iter().enumerate() {
            if i != j && aij.abs() > tol {
                return false;
            }
        }
    }
    true
}

pub fn is_upper_triangular(a: &[Vec<f64>], tol: f64) -> bool {
    for (i, ai) in a.iter().enumerate() {
        for &aij in &ai[..i] {
            if aij.abs() > tol {
                return false;
            }
        }
    }
    true
}

pub fn is_lower_triangular(a: &[Vec<f64>], tol: f64) -> bool {
    for (i, ai) in a.iter().enumerate() {
        for &aij in &ai[(i + 1)..] {
            if aij.abs() > tol {
                return false;
            }
        }
    }
    true
}

pub fn matrix_norm_1(a: &[Vec<f64>]) -> f64 {
    let n = a.len();
    let m = if n > 0 { a[0].len() } else { 0 };
    let mut max_col = 0.0f64;
    for j in 0..m {
        let col_sum: f64 = a.iter().map(|row| row[j].abs()).sum();
        if col_sum > max_col {
            max_col = col_sum;
        }
    }
    max_col
}

pub fn matrix_condition_frobenius(a: &[Vec<f64>]) -> f64 {
    let n = a.len();
    let mut inv = vec![vec![0.0; n]; n];
    let mut aug: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            let mut row = a[i].clone();
            row.push(0.0);
            row
        })
        .collect();
    for col_idx in 0..n {
        for (i, aug_i) in aug.iter_mut().enumerate() {
            aug_i[n] = if i == col_idx { 1.0 } else { 0.0 };
        }
        let mut work = aug.clone();
        for c in 0..n {
            let mut pivot = c;
            for r in (c + 1)..n {
                if work[r][c].abs() > work[pivot][c].abs() {
                    pivot = r;
                }
            }
            work.swap(c, pivot);
            if work[c][c].abs() < 1e-30 {
                continue;
            }
            for r in (c + 1)..n {
                let f = work[r][c] / work[c][c];
                let (top, bot) = work.split_at_mut(r);
                for (d, &s) in bot[0][c..=n].iter_mut().zip(&top[c][c..=n]) {
                    *d -= f * s;
                }
            }
        }
        for i in (0..n).rev() {
            let mut s = 0.0;
            for (&wij, inv_j) in work[i][(i + 1)..].iter().zip(&inv[(i + 1)..]) {
                s += wij * inv_j[col_idx];
            }
            inv[i][col_idx] = if work[i][i].abs() > 1e-30 {
                (work[i][n] - s) / work[i][i]
            } else {
                0.0
            };
        }
    }
    let norm_a = matrix_norm_frobenius(a);
    let norm_inv = matrix_norm_frobenius(&inv);
    norm_a * norm_inv
}

pub fn gram_matrix(a: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let m = a.len();
    let mut g = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in i..m {
            let mut dot = 0.0;
            for (&ai, &aj) in a[i].iter().zip(&a[j]) {
                dot += ai * aj;
            }
            g[i][j] = dot;
            g[j][i] = dot;
        }
    }
    g
}

pub fn column_space_basis(a: &[Vec<f64>], tol: f64) -> Vec<Vec<f64>> {
    let m = a.len();
    let n = if m > 0 {
        a[0].len()
    } else {
        return Vec::new();
    };
    let mut work = a.to_vec();
    let mut pivot_cols = Vec::new();
    let mut r = 0;
    for col in 0..n {
        let mut pivot = r;
        for row in (r + 1)..m {
            if work[row][col].abs() > work[pivot][col].abs() {
                pivot = row;
            }
        }
        if work[pivot][col].abs() < tol {
            continue;
        }
        work.swap(r, pivot);
        let div = work[r][col];
        work[r][col..n].iter_mut().for_each(|v| *v /= div);
        for row in 0..m {
            if row == r {
                continue;
            }
            let f = work[row][col];
            let src = work[r][col..n].to_vec();
            work[row][col..n]
                .iter_mut()
                .zip(&src)
                .for_each(|(d, &s)| *d -= f * s);
        }
        pivot_cols.push(col);
        r += 1;
    }
    pivot_cols
        .iter()
        .map(|&col| (0..m).map(|i| a[i][col]).collect())
        .collect()
}
