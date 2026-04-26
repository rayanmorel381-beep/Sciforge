use crate::constants::GOLDEN_RATIO_CONJUGATE;

pub fn gradient_descent(
    f: fn(&[f64]) -> f64,
    grad: fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    lr: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let mut x = x0.to_vec();
    for _ in 0..max_iter {
        let g = grad(&x);
        let norm: f64 = g.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm < tol {
            break;
        }
        for i in 0..x.len() {
            x[i] -= lr * g[i];
        }
    }
    let _ = f(&x);
    x
}

pub fn gradient_descent_momentum(
    grad: fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    lr: f64,
    momentum: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let mut x = x0.to_vec();
    let mut v = vec![0.0; x.len()];
    for _ in 0..max_iter {
        let g = grad(&x);
        let norm: f64 = g.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm < tol {
            break;
        }
        for i in 0..x.len() {
            v[i] = momentum * v[i] - lr * g[i];
            x[i] += v[i];
        }
    }
    x
}

pub fn adam(
    grad: fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    lr: f64,
    beta1: f64,
    beta2: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let mut x = x0.to_vec();
    let n = x.len();
    let mut m = vec![0.0; n];
    let mut v = vec![0.0; n];
    let eps = 1e-8;
    for t in 1..=max_iter {
        let g = grad(&x);
        let norm: f64 = g.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm < tol {
            break;
        }
        for i in 0..n {
            m[i] = beta1 * m[i] + (1.0 - beta1) * g[i];
            v[i] = beta2 * v[i] + (1.0 - beta2) * g[i] * g[i];
            let m_hat = m[i] / (1.0 - beta1.powi(t as i32));
            let v_hat = v[i] / (1.0 - beta2.powi(t as i32));
            x[i] -= lr * m_hat / (v_hat.sqrt() + eps);
        }
    }
    x
}

pub fn newton_method_1d(
    f: fn(f64) -> f64,
    df: fn(f64) -> f64,
    x0: f64,
    max_iter: usize,
    tol: f64,
) -> f64 {
    let mut x = x0;
    for _ in 0..max_iter {
        let fx = f(x);
        let dfx = df(x);
        if dfx.abs() < 1e-30 {
            break;
        }
        let x_new = x - fx / dfx;
        if (x_new - x).abs() < tol {
            return x_new;
        }
        x = x_new;
    }
    x
}

pub fn bisection(f: fn(f64) -> f64, mut a: f64, mut b: f64, tol: f64, max_iter: usize) -> f64 {
    for _ in 0..max_iter {
        let mid = (a + b) / 2.0;
        if (b - a) / 2.0 < tol {
            return mid;
        }
        if f(a) * f(mid) < 0.0 {
            b = mid;
        } else {
            a = mid;
        }
    }
    (a + b) / 2.0
}

pub fn secant_method(f: fn(f64) -> f64, x0: f64, x1: f64, max_iter: usize, tol: f64) -> f64 {
    let mut xn_1 = x0;
    let mut xn = x1;
    for _ in 0..max_iter {
        let fxn = f(xn);
        let fxn_1 = f(xn_1);
        if (fxn - fxn_1).abs() < 1e-30 {
            break;
        }
        let x_new = xn - fxn * (xn - xn_1) / (fxn - fxn_1);
        if (x_new - xn).abs() < tol {
            return x_new;
        }
        xn_1 = xn;
        xn = x_new;
    }
    xn
}

pub fn golden_section_search(f: fn(f64) -> f64, mut a: f64, mut b: f64, tol: f64) -> f64 {
    let mut c = b - GOLDEN_RATIO_CONJUGATE * (b - a);
    let mut d = a + GOLDEN_RATIO_CONJUGATE * (b - a);
    while (b - a).abs() > tol {
        if f(c) < f(d) {
            b = d;
        } else {
            a = c;
        }
        c = b - GOLDEN_RATIO_CONJUGATE * (b - a);
        d = a + GOLDEN_RATIO_CONJUGATE * (b - a);
    }
    (a + b) / 2.0
}

pub fn numerical_gradient(f: fn(&[f64]) -> f64, x: &[f64], h: f64) -> Vec<f64> {
    let n = x.len();
    let mut grad = vec![0.0; n];
    for i in 0..n {
        let mut xp = x.to_vec();
        let mut xm = x.to_vec();
        xp[i] += h;
        xm[i] -= h;
        grad[i] = (f(&xp) - f(&xm)) / (2.0 * h);
    }
    grad
}

pub fn nesterov_momentum(
    grad: fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    lr: f64,
    momentum: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let mut x = x0.to_vec();
    let n = x.len();
    let mut v = vec![0.0; n];
    for _ in 0..max_iter {
        let lookahead: Vec<f64> = (0..n).map(|i| x[i] + momentum * v[i]).collect();
        let g = grad(&lookahead);
        let norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if norm < tol {
            break;
        }
        for i in 0..n {
            v[i] = momentum * v[i] - lr * g[i];
            x[i] += v[i];
        }
    }
    x
}

pub fn rmsprop(
    grad: fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    lr: f64,
    decay: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let mut x = x0.to_vec();
    let n = x.len();
    let mut cache = vec![0.0; n];
    let eps = 1e-8;
    for _ in 0..max_iter {
        let g = grad(&x);
        let norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if norm < tol {
            break;
        }
        for i in 0..n {
            cache[i] = decay * cache[i] + (1.0 - decay) * g[i] * g[i];
            x[i] -= lr * g[i] / (cache[i].sqrt() + eps);
        }
    }
    x
}

pub fn adagrad(
    grad: fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    lr: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let mut x = x0.to_vec();
    let n = x.len();
    let mut accum = vec![0.0; n];
    let eps = 1e-8;
    for _ in 0..max_iter {
        let g = grad(&x);
        let norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if norm < tol {
            break;
        }
        for i in 0..n {
            accum[i] += g[i] * g[i];
            x[i] -= lr * g[i] / (accum[i].sqrt() + eps);
        }
    }
    x
}

pub fn line_search_backtracking(
    f: fn(&[f64]) -> f64,
    x: &[f64],
    direction: &[f64],
    alpha0: f64,
    c: f64,
    rho: f64,
) -> f64 {
    let n = x.len();
    let fx = f(x);
    let grad_dot: f64 = {
        let h = 1e-7;
        (0..n)
            .map(|i| {
                let mut xp = x.to_vec();
                let mut xm = x.to_vec();
                xp[i] += h;
                xm[i] -= h;
                let gi = (f(&xp) - f(&xm)) / (2.0 * h);
                gi * direction[i]
            })
            .sum()
    };
    let mut alpha = alpha0;
    for _ in 0..50 {
        let x_new: Vec<f64> = (0..n).map(|i| x[i] + alpha * direction[i]).collect();
        if f(&x_new) <= fx + c * alpha * grad_dot {
            break;
        }
        alpha *= rho;
    }
    alpha
}

pub fn bfgs(
    f: fn(&[f64]) -> f64,
    grad: fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut h = vec![vec![0.0; n]; n];
    for (i, hi) in h.iter_mut().enumerate() {
        hi[i] = 1.0;
    }
    let mut g = grad(&x);
    for _ in 0..max_iter {
        let norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if norm < tol {
            break;
        }
        let direction: Vec<f64> = (0..n)
            .map(|i| -(0..n).map(|j| h[i][j] * g[j]).sum::<f64>())
            .collect();
        let alpha = line_search_backtracking(f, &x, &direction, 1.0, 1e-4, 0.5);
        let s: Vec<f64> = (0..n).map(|i| alpha * direction[i]).collect();
        let x_new: Vec<f64> = (0..n).map(|i| x[i] + s[i]).collect();
        let g_new = grad(&x_new);
        let y: Vec<f64> = (0..n).map(|i| g_new[i] - g[i]).collect();
        let sy: f64 = s.iter().zip(y.iter()).map(|(si, yi)| si * yi).sum();
        if sy.abs() < 1e-30 {
            x = x_new;
            g = g_new;
            continue;
        }
        let mut hs = vec![0.0; n];
        for (i, hsi) in hs.iter_mut().enumerate() {
            for (j, &yj) in y.iter().enumerate() {
                *hsi += h[i][j] * yj;
            }
        }
        let yhy: f64 = y.iter().zip(hs.iter()).map(|(yi, hi)| yi * hi).sum();
        for (i, hi) in h.iter_mut().enumerate() {
            for j in 0..n {
                hi[j] += (sy + yhy) * s[i] * s[j] / (sy * sy) - (hs[i] * s[j] + s[i] * hs[j]) / sy;
            }
        }
        x = x_new;
        g = g_new;
    }
    x
}

pub fn conjugate_gradient_min(
    grad: fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut g = grad(&x);
    let mut d: Vec<f64> = g.iter().map(|gi| -gi).collect();
    for _ in 0..max_iter {
        let g_norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>();
        if g_norm.sqrt() < tol {
            break;
        }
        let alpha = {
            let mut a = 0.001;
            for _ in 0..20 {
                let x1: Vec<f64> = (0..n).map(|i| x[i] + a * d[i]).collect();
                let g1 = grad(&x1);
                let dg: f64 = d.iter().zip(g1.iter()).map(|(di, gi)| di * gi).sum();
                if dg.abs() < tol {
                    break;
                }
                a *= 0.5;
            }
            a
        };
        for i in 0..n {
            x[i] += alpha * d[i];
        }
        let g_new = grad(&x);
        let g_new_norm: f64 = g_new.iter().map(|gi| gi * gi).sum::<f64>();
        let beta = g_new_norm / g_norm;
        d = (0..n).map(|i| -g_new[i] + beta * d[i]).collect();
        g = g_new;
    }
    x
}

pub fn hessian_numerical(f: fn(&[f64]) -> f64, x: &[f64], h: f64) -> Vec<Vec<f64>> {
    let n = x.len();
    let mut hess = vec![vec![0.0; n]; n];
    let fx = f(x);
    for i in 0..n {
        for j in i..n {
            let mut xpp = x.to_vec();
            let mut xpm = x.to_vec();
            let mut xmp = x.to_vec();
            let mut xmm = x.to_vec();
            xpp[i] += h;
            xpp[j] += h;
            xpm[i] += h;
            xpm[j] -= h;
            xmp[i] -= h;
            xmp[j] += h;
            xmm[i] -= h;
            xmm[j] -= h;
            let val = (f(&xpp) - f(&xpm) - f(&xmp) + f(&xmm)) / (4.0 * h * h);
            hess[i][j] = val;
            hess[j][i] = val;
        }
    }
    let _ = fx;
    hess
}

pub fn ternary_search(f: fn(f64) -> f64, mut lo: f64, mut hi: f64, tol: f64) -> f64 {
    while (hi - lo).abs() > tol {
        let m1 = lo + (hi - lo) / 3.0;
        let m2 = hi - (hi - lo) / 3.0;
        if f(m1) < f(m2) {
            hi = m2;
        } else {
            lo = m1;
        }
    }
    (lo + hi) / 2.0
}

pub fn newton_method_nd(f: fn(&[f64]) -> f64, x0: &[f64], max_iter: usize, tol: f64) -> Vec<f64> {
    let n = x0.len();
    let mut x = x0.to_vec();
    let h = 1e-6;
    for _ in 0..max_iter {
        let g = numerical_gradient(f, &x, h);
        let norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if norm < tol {
            break;
        }
        let hess = hessian_numerical(f, &x, h);
        let dx = solve_linear_system(&hess, &g);
        for i in 0..n {
            x[i] -= dx[i];
        }
    }
    x
}

fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut aug: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            let mut row = a[i].clone();
            row.push(b[i]);
            row
        })
        .collect();
    for col in 0..n {
        let mut pivot = col;
        for row in (col + 1)..n {
            if aug[row][col].abs() > aug[pivot][col].abs() {
                pivot = row;
            }
        }
        aug.swap(col, pivot);
        if aug[col][col].abs() < 1e-30 {
            continue;
        }
        for row in (col + 1)..n {
            let factor = aug[row][col] / aug[col][col];
            let (top, bot) = aug.split_at_mut(row);
            for (d, &s) in bot[0][col..=n].iter_mut().zip(&top[col][col..=n]) {
                *d -= factor * s;
            }
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = aug[i][n];
        for j in (i + 1)..n {
            x[i] -= aug[i][j] * x[j];
        }
        if aug[i][i].abs() > 1e-30 {
            x[i] /= aug[i][i];
        }
    }
    x
}
