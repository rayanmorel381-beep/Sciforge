use super::poly::Polynomial;

pub fn newton_raphson(
    f: impl Fn(f64) -> f64,
    df: impl Fn(f64) -> f64,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> f64 {
    let mut x = x0;
    for _ in 0..max_iter {
        let fx = f(x);
        let dfx = df(x);
        if dfx.abs() < 1e-30 {
            break;
        }
        let xn = x - fx / dfx;
        if (xn - x).abs() < tol {
            return xn;
        }
        x = xn;
    }
    x
}

pub fn bisection(f: impl Fn(f64) -> f64, mut a: f64, mut b: f64, tol: f64, max_iter: usize) -> f64 {
    for _ in 0..max_iter {
        let mid = (a + b) * 0.5;
        if (b - a) < tol {
            return mid;
        }
        if f(a) * f(mid) < 0.0 {
            b = mid;
        } else {
            a = mid;
        }
    }
    (a + b) * 0.5
}

pub fn secant_method(
    f: impl Fn(f64) -> f64,
    mut x0: f64,
    mut x1: f64,
    tol: f64,
    max_iter: usize,
) -> f64 {
    for _ in 0..max_iter {
        let f0 = f(x0);
        let f1 = f(x1);
        if (f1 - f0).abs() < 1e-30 {
            break;
        }
        let x2 = x1 - f1 * (x1 - x0) / (f1 - f0);
        if (x2 - x1).abs() < tol {
            return x2;
        }
        x0 = x1;
        x1 = x2;
    }
    x1
}

pub fn brent_method(
    f: impl Fn(f64) -> f64,
    mut a: f64,
    mut b: f64,
    tol: f64,
    max_iter: usize,
) -> f64 {
    let mut fa = f(a);
    let mut fb = f(b);
    if fa * fb > 0.0 {
        return (a + b) * 0.5;
    }
    if fa.abs() < fb.abs() {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut fa, &mut fb);
    }
    let mut c = a;
    let mut fc = fa;
    let mut mflag = true;
    let mut d = 0.0;

    for _ in 0..max_iter {
        if fb.abs() < tol {
            return b;
        }
        if (b - a).abs() < tol {
            return b;
        }

        let s = if (fa - fc).abs() > 1e-30 && (fb - fc).abs() > 1e-30 {
            a * fb * fc / ((fa - fb) * (fa - fc))
                + b * fa * fc / ((fb - fa) * (fb - fc))
                + c * fa * fb / ((fc - fa) * (fc - fb))
        } else {
            b - fb * (b - a) / (fb - fa)
        };

        let cond1 = if a < b {
            s < (3.0 * a + b) / 4.0 || s > b
        } else {
            s > (3.0 * a + b) / 4.0 || s < b
        };
        let cond2 = mflag && (s - b).abs() >= (b - c).abs() * 0.5;
        let cond3 = !mflag && (s - b).abs() >= (c - d).abs() * 0.5;
        let cond4 = mflag && (b - c).abs() < tol;
        let cond5 = !mflag && (c - d).abs() < tol;

        if cond1 || cond2 || cond3 || cond4 || cond5 {
            let s_mid = (a + b) * 0.5;
            mflag = true;
            d = c;
            c = b;
            fc = fb;
            let fs = f(s_mid);
            if fa * fs < 0.0 {
                b = s_mid;
                fb = fs;
            } else {
                a = s_mid;
                fa = fs;
            }
        } else {
            mflag = false;
            d = c;
            c = b;
            fc = fb;
            let fs = f(s);
            if fa * fs < 0.0 {
                b = s;
                fb = fs;
            } else {
                a = s;
                fa = fs;
            }
        }

        if fa.abs() < fb.abs() {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut fa, &mut fb);
        }
    }
    b
}

pub fn polynomial_roots_real(
    poly: &Polynomial,
    search_range: (f64, f64),
    subdivisions: usize,
    tol: f64,
) -> Vec<f64> {
    let (lo, hi) = search_range;
    let step = (hi - lo) / subdivisions as f64;
    let mut roots = vec![];
    let dpoly = poly.derivative();

    for i in 0..subdivisions {
        let a = lo + i as f64 * step;
        let b = a + step;
        let fa = poly.eval(a);
        let fb = poly.eval(b);
        if fa * fb <= 0.0 {
            let root = newton_raphson(|x| poly.eval(x), |x| dpoly.eval(x), (a + b) * 0.5, tol, 100);
            if roots
                .last()
                .is_none_or(|&last: &f64| (last - root).abs() > tol * 10.0)
            {
                roots.push(root);
            }
        }
    }
    roots
}

pub fn durand_kerner(poly: &Polynomial, max_iter: usize, tol: f64) -> Vec<(f64, f64)> {
    let n = poly.degree();
    if n == 0 {
        return vec![];
    }
    let lead = poly.coeffs[n];
    let mut roots: Vec<(f64, f64)> = (0..n)
        .map(|k| {
            let angle = 2.0 * std::f64::consts::PI * k as f64 / n as f64 + 0.4;
            let r = 1.0 + poly.coeffs.iter().map(|c| c.abs()).sum::<f64>() / lead.abs();
            (r * angle.cos(), r * angle.sin())
        })
        .collect();

    for _ in 0..max_iter {
        let mut max_delta: f64 = 0.0;
        for i in 0..n {
            let (zr, zi) = roots[i];
            let mut pr = 0.0;
            let mut pi = 0.0;
            let mut zp_r = 1.0;
            let mut zp_i = 0.0;
            for &c in &poly.coeffs {
                pr += c * zp_r;
                pi += c * zp_i;
                let new_zp_r = zp_r * zr - zp_i * zi;
                let new_zp_i = zp_r * zi + zp_i * zr;
                zp_r = new_zp_r;
                zp_i = new_zp_i;
            }

            let mut denom_r = 1.0;
            let mut denom_i = 0.0;
            for (j, &rj) in roots.iter().enumerate() {
                if j == i {
                    continue;
                }
                let dr = zr - rj.0;
                let di = zi - rj.1;
                let new_r = denom_r * dr - denom_i * di;
                let new_i = denom_r * di + denom_i * dr;
                denom_r = new_r;
                denom_i = new_i;
            }

            let d_sq = denom_r * denom_r + denom_i * denom_i;
            if d_sq < 1e-60 {
                continue;
            }
            let delta_r = (pr * denom_r + pi * denom_i) / d_sq;
            let delta_i = (pi * denom_r - pr * denom_i) / d_sq;
            roots[i] = (zr - delta_r, zi - delta_i);
            max_delta = max_delta.max((delta_r * delta_r + delta_i * delta_i).sqrt());
        }
        if max_delta < tol {
            break;
        }
    }
    roots
}

pub fn ridder_method(
    f: impl Fn(f64) -> f64,
    mut a: f64,
    mut b: f64,
    tol: f64,
    max_iter: usize,
) -> f64 {
    for _ in 0..max_iter {
        let fa = f(a);
        let fb = f(b);
        let mid = (a + b) * 0.5;
        let fm = f(mid);
        let s = (fm * fm - fa * fb).sqrt();
        if s < 1e-30 {
            return mid;
        }
        let sign = if (fa - fb) > 0.0 { 1.0 } else { -1.0 };
        let xn = mid + (mid - a) * sign * fm / s;
        if (xn - mid).abs() < tol {
            return xn;
        }
        let fn_val = f(xn);
        if fn_val.abs() < tol {
            return xn;
        }
        if fm * fn_val < 0.0 {
            a = mid;
            b = xn;
        } else if fa * fn_val < 0.0 {
            b = xn;
        } else {
            a = xn;
        }
    }
    (a + b) * 0.5
}

pub fn illinois_method(
    f: impl Fn(f64) -> f64,
    mut a: f64,
    mut b: f64,
    tol: f64,
    max_iter: usize,
) -> f64 {
    let mut fa = f(a);
    let mut fb = f(b);
    for _ in 0..max_iter {
        let c = b - fb * (b - a) / (fb - fa);
        let fc = f(c);
        if fc.abs() < tol || (b - a).abs() < tol {
            return c;
        }
        if fc * fb < 0.0 {
            a = b;
            fa = fb;
        } else {
            fa *= 0.5;
        }
        b = c;
        fb = fc;
    }
    b
}

pub fn muller_method(
    f: impl Fn(f64) -> f64,
    x0: f64,
    x1: f64,
    x2: f64,
    tol: f64,
    max_iter: usize,
) -> f64 {
    let mut xn = [x0, x1, x2];
    for _ in 0..max_iter {
        let f0 = f(xn[0]);
        let f1 = f(xn[1]);
        let f2 = f(xn[2]);
        let h0 = xn[1] - xn[0];
        let h1 = xn[2] - xn[1];
        let d0 = (f1 - f0) / h0;
        let d1 = (f2 - f1) / h1;
        let a = (d1 - d0) / (h1 + h0);
        let b = a * h1 + d1;
        let c = f2;
        let disc = (b * b - 4.0 * a * c).max(0.0).sqrt();
        let denom = if (b + disc).abs() > (b - disc).abs() {
            b + disc
        } else {
            b - disc
        };
        if denom.abs() < 1e-30 {
            return xn[2];
        }
        let dx = -2.0 * c / denom;
        let x3 = xn[2] + dx;
        if dx.abs() < tol {
            return x3;
        }
        xn[0] = xn[1];
        xn[1] = xn[2];
        xn[2] = x3;
    }
    xn[2]
}

pub fn fixed_point_iteration(g: impl Fn(f64) -> f64, x0: f64, tol: f64, max_iter: usize) -> f64 {
    let mut x = x0;
    for _ in 0..max_iter {
        let xn = g(x);
        if (xn - x).abs() < tol {
            return xn;
        }
        x = xn;
    }
    x
}

pub fn steffensen_method(f: impl Fn(f64) -> f64, x0: f64, tol: f64, max_iter: usize) -> f64 {
    let mut x = x0;
    for _ in 0..max_iter {
        let fx = f(x);
        if fx.abs() < tol {
            return x;
        }
        let gx = f(x + fx);
        let denom = gx - fx;
        if denom.abs() < 1e-30 {
            return x;
        }
        let xn = x - fx * fx / denom;
        if (xn - x).abs() < tol {
            return xn;
        }
        x = xn;
    }
    x
}

pub fn halley_method(
    f: impl Fn(f64) -> f64,
    df: impl Fn(f64) -> f64,
    ddf: impl Fn(f64) -> f64,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> f64 {
    let mut x = x0;
    for _ in 0..max_iter {
        let fx = f(x);
        let dfx = df(x);
        let ddfx = ddf(x);
        let denom = 2.0 * dfx * dfx - fx * ddfx;
        if denom.abs() < 1e-30 {
            break;
        }
        let xn = x - 2.0 * fx * dfx / denom;
        if (xn - x).abs() < tol {
            return xn;
        }
        x = xn;
    }
    x
}

pub fn laguerre_root(poly: &Polynomial, x0: f64, tol: f64, max_iter: usize) -> f64 {
    let n = poly.degree() as f64;
    let dp = poly.derivative();
    let ddp = dp.derivative();
    let mut x = x0;
    for _ in 0..max_iter {
        let px = poly.eval(x);
        if px.abs() < tol {
            return x;
        }
        let dpx = dp.eval(x);
        let ddpx = ddp.eval(x);
        let g = dpx / px;
        let h = g * g - ddpx / px;
        let disc = ((n - 1.0) * (n * h - g * g)).abs().sqrt();
        let denom = if (g + disc).abs() > (g - disc).abs() {
            g + disc
        } else {
            g - disc
        };
        if denom.abs() < 1e-30 {
            break;
        }
        let a = n / denom;
        x -= a;
        if a.abs() < tol {
            return x;
        }
    }
    x
}

pub fn polynomial_deflate(poly: &Polynomial, root: f64) -> Polynomial {
    let n = poly.degree();
    let mut result = vec![0.0; n];
    result[n - 1] = poly.coeffs[n];
    for i in (0..n - 1).rev() {
        result[i] = poly.coeffs[i + 1] + root * result[i + 1];
    }
    Polynomial::new(result)
}

pub fn find_all_roots_real(poly: &Polynomial, tol: f64, max_iter: usize) -> Vec<f64> {
    let n = poly.degree();
    if n == 0 {
        return vec![];
    }
    let mut roots = Vec::new();
    let mut current = poly.clone();
    for _ in 0..n {
        if current.degree() == 0 {
            break;
        }
        let bound = 1.0
            + current.coeffs.iter().map(|c| c.abs()).sum::<f64>()
                / current.coeffs[current.degree()].abs();
        let root = laguerre_root(&current, bound * 0.5, tol, max_iter);
        roots.push(root);
        current = polynomial_deflate(&current, root);
    }
    roots
}
