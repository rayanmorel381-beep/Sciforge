pub fn shooting_method(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    ya: f64,
    yb: f64,
    dt: f64,
    tol: f64,
    max_iter: usize,
) -> Vec<(f64, f64)> {
    let mut s0 = (yb - ya) / (t_span.1 - t_span.0);
    let mut s1 = s0 + 1.0;

    let integrate = |s: f64| -> f64 {
        let mut t = t_span.0;
        let mut y = vec![ya, s];
        while t < t_span.1 - 1e-12 {
            let h = dt.min(t_span.1 - t);
            let k1 = f(t, &y);
            let y2: Vec<f64> = y
                .iter()
                .zip(&k1)
                .map(|(yi, ki)| yi + 0.5 * h * ki)
                .collect();
            let k2 = f(t + 0.5 * h, &y2);
            let y3: Vec<f64> = y
                .iter()
                .zip(&k2)
                .map(|(yi, ki)| yi + 0.5 * h * ki)
                .collect();
            let k3 = f(t + 0.5 * h, &y3);
            let y4: Vec<f64> = y.iter().zip(&k3).map(|(yi, ki)| yi + h * ki).collect();
            let k4 = f(t + h, &y4);
            for (i, yi) in y.iter_mut().enumerate() {
                *yi += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
            }
            t += h;
        }
        y[0]
    };

    let mut f0 = integrate(s0) - yb;
    for _ in 0..max_iter {
        let f1 = integrate(s1) - yb;
        if f1.abs() < tol {
            let mut t = t_span.0;
            let mut y = vec![ya, s1];
            let mut result = vec![(t, y[0])];
            while t < t_span.1 - 1e-12 {
                let h = dt.min(t_span.1 - t);
                let k1 = f(t, &y);
                let y2: Vec<f64> = y
                    .iter()
                    .zip(&k1)
                    .map(|(yi, ki)| yi + 0.5 * h * ki)
                    .collect();
                let k2 = f(t + 0.5 * h, &y2);
                let y3: Vec<f64> = y
                    .iter()
                    .zip(&k2)
                    .map(|(yi, ki)| yi + 0.5 * h * ki)
                    .collect();
                let k3 = f(t + 0.5 * h, &y3);
                let y4: Vec<f64> = y.iter().zip(&k3).map(|(yi, ki)| yi + h * ki).collect();
                let k4 = f(t + h, &y4);
                for (i, yi) in y.iter_mut().enumerate() {
                    *yi += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
                }
                t += h;
                result.push((t, y[0]));
            }
            return result;
        }
        if (f1 - f0).abs() < 1e-30 {
            break;
        }
        let s2 = s1 - f1 * (s1 - s0) / (f1 - f0);
        s0 = s1;
        f0 = f1;
        s1 = s2;
    }
    vec![]
}

pub fn finite_difference_bvp(
    p: impl Fn(f64) -> f64,
    q: impl Fn(f64) -> f64,
    r: impl Fn(f64) -> f64,
    x_range: (f64, f64),
    ya: f64,
    yb: f64,
    n: usize,
) -> Vec<(f64, f64)> {
    let h = (x_range.1 - x_range.0) / (n + 1) as f64;
    let m = n;
    let mut a_mat = vec![vec![0.0; m]; m];
    let mut b_vec = vec![0.0; m];

    for i in 0..m {
        let x = x_range.0 + (i + 1) as f64 * h;
        let pi = p(x);
        let qi = q(x);
        let ri = r(x);

        a_mat[i][i] = -2.0 + h * h * qi;
        if i > 0 {
            a_mat[i][i - 1] = 1.0 - 0.5 * h * pi;
        }
        if i < m - 1 {
            a_mat[i][i + 1] = 1.0 + 0.5 * h * pi;
        }

        b_vec[i] = h * h * ri;
        if i == 0 {
            b_vec[i] -= (1.0 - 0.5 * h * pi) * ya;
        }
        if i == m - 1 {
            b_vec[i] -= (1.0 + 0.5 * h * pi) * yb;
        }
    }

    let y_internal = tridiag_solve(&a_mat, &b_vec);

    let mut result = vec![(x_range.0, ya)];
    for (i, &yi) in y_internal.iter().enumerate() {
        result.push((x_range.0 + (i + 1) as f64 * h, yi));
    }
    result.push((x_range.1, yb));
    result
}

pub fn collocation_bvp(
    f: impl Fn(f64, f64, f64) -> f64,
    x_range: (f64, f64),
    ya: f64,
    yb: f64,
    n: usize,
    max_iter: usize,
    tol: f64,
) -> Vec<(f64, f64)> {
    let h = (x_range.1 - x_range.0) / (n + 1) as f64;
    let mut y = vec![0.0; n];
    for (i, yi) in y.iter_mut().enumerate() {
        let t = (i + 1) as f64 / (n + 1) as f64;
        *yi = ya * (1.0 - t) + yb * t;
    }
    for _ in 0..max_iter {
        let mut residual = vec![0.0; n];
        for i in 0..n {
            let x = x_range.0 + (i + 1) as f64 * h;
            let y_prev = if i == 0 { ya } else { y[i - 1] };
            let y_next = if i == n - 1 { yb } else { y[i + 1] };
            let ypp = (y_prev - 2.0 * y[i] + y_next) / (h * h);
            let yp = (y_next - y_prev) / (2.0 * h);
            residual[i] = ypp - f(x, y[i], yp);
        }
        let max_res: f64 = residual.iter().map(|r| r.abs()).fold(0.0, f64::max);
        if max_res < tol {
            break;
        }
        for (yi, &ri) in y.iter_mut().zip(residual.iter()) {
            *yi += 0.5 * h * h * ri;
        }
    }
    let mut result = vec![(x_range.0, ya)];
    for (i, &yi) in y.iter().enumerate() {
        result.push((x_range.0 + (i + 1) as f64 * h, yi));
    }
    result.push((x_range.1, yb));
    result
}

pub fn multiple_shooting(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    ya: f64,
    yb: f64,
    segments: usize,
    dt: f64,
    tol: f64,
    max_iter: usize,
) -> Vec<(f64, f64)> {
    let seg_len = (t_span.1 - t_span.0) / segments as f64;
    let mut slopes = vec![(yb - ya) / (t_span.1 - t_span.0); segments];
    for _ in 0..max_iter {
        let mut all_good = true;
        for seg in 0..segments {
            let t0 = t_span.0 + seg as f64 * seg_len;
            let t1 = t0 + seg_len;
            let y0_seg = if seg == 0 {
                ya
            } else {
                ya + slopes[..seg].iter().sum::<f64>() * seg_len
            };
            let mut t = t0;
            let mut y = vec![y0_seg, slopes[seg]];
            while t < t1 - 1e-12 {
                let h = dt.min(t1 - t);
                let k1 = f(t, &y);
                let y2: Vec<f64> = y
                    .iter()
                    .zip(&k1)
                    .map(|(yi, ki)| yi + 0.5 * h * ki)
                    .collect();
                let k2 = f(t + 0.5 * h, &y2);
                let y3: Vec<f64> = y
                    .iter()
                    .zip(&k2)
                    .map(|(yi, ki)| yi + 0.5 * h * ki)
                    .collect();
                let k3 = f(t + 0.5 * h, &y3);
                let y4: Vec<f64> = y.iter().zip(&k3).map(|(yi, ki)| yi + h * ki).collect();
                let k4 = f(t + h, &y4);
                for (i, yi) in y.iter_mut().enumerate() {
                    *yi += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
                }
                t += h;
            }
            let target = if seg == segments - 1 {
                yb
            } else {
                ya + slopes[..=seg].iter().sum::<f64>() * seg_len
            };
            let err = y[0] - target;
            if err.abs() > tol {
                all_good = false;
            }
            slopes[seg] -= err / seg_len;
        }
        if all_good {
            break;
        }
    }
    shooting_method(&f, t_span, ya, yb, dt, tol, max_iter)
}

pub fn sturm_liouville_eigenvalues(
    p: impl Fn(f64) -> f64,
    q: impl Fn(f64) -> f64,
    w: impl Fn(f64) -> f64,
    x_range: (f64, f64),
    n: usize,
    num_eigenvalues: usize,
) -> Vec<f64> {
    let h = (x_range.1 - x_range.0) / (n + 1) as f64;
    let m = n;
    let mut a_mat = vec![vec![0.0; m]; m];
    let mut b_mat = vec![vec![0.0; m]; m];
    for i in 0..m {
        let x = x_range.0 + (i + 1) as f64 * h;
        let pi = p(x);
        let qi = q(x);
        let wi = w(x);
        a_mat[i][i] = 2.0 * pi / (h * h) + qi;
        if i > 0 {
            a_mat[i][i - 1] = -pi / (h * h);
        }
        if i < m - 1 {
            a_mat[i][i + 1] = -pi / (h * h);
        }
        b_mat[i][i] = wi;
    }
    let mut eigenvalues = Vec::new();
    let mut lambda = 0.1;
    for _ in 0..num_eigenvalues {
        for _ in 0..100 {
            let mut mat = vec![vec![0.0; m]; m];
            for (i, mat_i) in mat.iter_mut().enumerate() {
                for (j, mat_ij) in mat_i.iter_mut().enumerate() {
                    *mat_ij = a_mat[i][j] - lambda * b_mat[i][j];
                }
            }
            let det = matrix_det_small(&mat);
            let eps = 1e-6;
            let mut mat2 = mat.clone();
            for (i, m2i) in mat2.iter_mut().enumerate() {
                m2i[i] -= eps;
            }
            let det2 = matrix_det_small(&mat2);
            let ddet = (det2 - det) / (-eps);
            if ddet.abs() < 1e-30 {
                break;
            }
            let correction = det / ddet;
            lambda -= correction;
            if correction.abs() < 1e-10 {
                break;
            }
        }
        eigenvalues.push(lambda);
        lambda += 1.0;
    }
    eigenvalues
}

fn matrix_det_small(m: &[Vec<f64>]) -> f64 {
    let n = m.len();
    let mut lu = m.to_vec();
    let mut det = 1.0;
    for j in 0..n {
        for i in j + 1..n {
            if lu[j][j].abs() < 1e-30 {
                return 0.0;
            }
            let factor = lu[i][j] / lu[j][j];
            let (top, bot) = lu.split_at_mut(i);
            for (d, &s) in bot[0][j..n].iter_mut().zip(&top[j][j..n]) {
                *d -= factor * s;
            }
        }
        det *= lu[j][j];
    }
    det
}

pub fn relaxation_bvp(
    f: impl Fn(f64, f64) -> f64,
    x_range: (f64, f64),
    ya: f64,
    yb: f64,
    n: usize,
    omega: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<(f64, f64)> {
    let h = (x_range.1 - x_range.0) / (n + 1) as f64;
    let mut y = vec![0.0; n + 2];
    y[0] = ya;
    y[n + 1] = yb;
    for (i, yi) in y.iter_mut().enumerate().skip(1).take(n) {
        let t = i as f64 / (n + 1) as f64;
        *yi = ya * (1.0 - t) + yb * t;
    }
    for _ in 0..max_iter {
        let mut max_change = 0.0_f64;
        for i in 1..=n {
            let x = x_range.0 + i as f64 * h;
            let y_new = 0.5 * (y[i - 1] + y[i + 1] - h * h * f(x, y[i]));
            let change = (y_new - y[i]).abs();
            max_change = max_change.max(change);
            y[i] = (1.0 - omega) * y[i] + omega * y_new;
        }
        if max_change < tol {
            break;
        }
    }
    (0..=n + 1)
        .map(|i| (x_range.0 + i as f64 * h, y[i]))
        .collect()
}

fn tridiag_solve(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut c = vec![0.0; n];
    let mut d = b.to_vec();

    c[0] = a[0][1] / a[0][0];
    d[0] /= a[0][0];

    for i in 1..n {
        let sub = if i > 0 { a[i][i - 1] } else { 0.0 };
        let sup = if i < n - 1 { a[i][i + 1] } else { 0.0 };
        let m = a[i][i] - sub * c[i - 1];
        if i < n - 1 {
            c[i] = sup / m;
        }
        d[i] = (d[i] - sub * d[i - 1]) / m;
    }

    let mut x = vec![0.0; n];
    x[n - 1] = d[n - 1];
    for i in (0..n - 1).rev() {
        x[i] = d[i] - c[i] * x[i + 1];
    }
    x
}
