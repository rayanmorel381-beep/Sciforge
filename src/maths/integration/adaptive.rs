pub fn adaptive_simpson(f: impl Fn(f64) -> f64, a: f64, b: f64, tol: f64, max_depth: usize) -> f64 {
    adaptive_simpson_rec(&f, a, b, tol, max_depth, 0)
}

fn adaptive_simpson_rec(
    f: &impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_depth: usize,
    depth: usize,
) -> f64 {
    let mid = 0.5 * (a + b);
    let h = b - a;
    let s_whole = h / 6.0 * (f(a) + 4.0 * f(mid) + f(b));
    let s_left = (mid - a) / 6.0 * (f(a) + 4.0 * f(0.5 * (a + mid)) + f(mid));
    let s_right = (b - mid) / 6.0 * (f(mid) + 4.0 * f(0.5 * (mid + b)) + f(b));
    let s_split = s_left + s_right;

    if depth >= max_depth || (s_split - s_whole).abs() < 15.0 * tol {
        return s_split + (s_split - s_whole) / 15.0;
    }

    adaptive_simpson_rec(f, a, mid, tol * 0.5, max_depth, depth + 1)
        + adaptive_simpson_rec(f, mid, b, tol * 0.5, max_depth, depth + 1)
}

pub fn adaptive_trapezoid(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_depth: usize,
) -> f64 {
    adaptive_trapezoid_rec(&f, a, b, tol, max_depth, 0)
}

fn adaptive_trapezoid_rec(
    f: &impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_depth: usize,
    depth: usize,
) -> f64 {
    let mid = 0.5 * (a + b);
    let h = b - a;
    let whole = 0.5 * h * (f(a) + f(b));
    let left = 0.5 * (mid - a) * (f(a) + f(mid));
    let right = 0.5 * (b - mid) * (f(mid) + f(b));
    let split = left + right;

    if depth >= max_depth || (split - whole).abs() < 3.0 * tol {
        return split + (split - whole) / 3.0;
    }

    adaptive_trapezoid_rec(f, a, mid, tol * 0.5, max_depth, depth + 1)
        + adaptive_trapezoid_rec(f, mid, b, tol * 0.5, max_depth, depth + 1)
}

pub fn improper_integral_transform(f: impl Fn(f64) -> f64, a: f64, n: usize) -> f64 {
    let g = |t: f64| {
        if t.abs() < 1e-30 {
            return 0.0;
        }
        let x = a + (1.0 - t) / t;
        f(x) / (t * t)
    };
    super::quadrature::simpson(g, 1e-10, 1.0, n)
}

pub fn adaptive_gauss_kronrod(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_depth: usize,
) -> f64 {
    adaptive_gk_rec(&f, a, b, tol, max_depth, 0)
}

fn adaptive_gk_rec(
    f: &impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_depth: usize,
    depth: usize,
) -> f64 {
    let (val, err) = super::quadrature::gauss_kronrod_15(f, a, b);
    if depth >= max_depth || err < tol {
        return val;
    }
    let mid = 0.5 * (a + b);
    adaptive_gk_rec(f, a, mid, tol * 0.5, max_depth, depth + 1)
        + adaptive_gk_rec(f, mid, b, tol * 0.5, max_depth, depth + 1)
}

pub fn adaptive_midpoint(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_depth: usize,
) -> f64 {
    adaptive_midpoint_rec(&f, a, b, tol, max_depth, 0)
}

fn adaptive_midpoint_rec(
    f: &impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_depth: usize,
    depth: usize,
) -> f64 {
    let mid = 0.5 * (a + b);
    let h = b - a;
    let whole = h * f(mid);
    let left = 0.5 * h * f(0.5 * (a + mid));
    let right = 0.5 * h * f(0.5 * (mid + b));
    let split = left + right;
    if depth >= max_depth || (split - whole).abs() < 3.0 * tol {
        return split;
    }
    adaptive_midpoint_rec(f, a, mid, tol * 0.5, max_depth, depth + 1)
        + adaptive_midpoint_rec(f, mid, b, tol * 0.5, max_depth, depth + 1)
}

pub fn adaptive_boole(f: impl Fn(f64) -> f64, a: f64, b: f64, tol: f64, max_depth: usize) -> f64 {
    adaptive_boole_rec(&f, a, b, tol, max_depth, 0)
}

fn adaptive_boole_rec(
    f: &impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_depth: usize,
    depth: usize,
) -> f64 {
    let h = b - a;
    let x0 = a;
    let x1 = a + 0.25 * h;
    let x2 = a + 0.5 * h;
    let x3 = a + 0.75 * h;
    let x4 = b;
    let whole = h / 90.0 * (7.0 * f(x0) + 32.0 * f(x1) + 12.0 * f(x2) + 32.0 * f(x3) + 7.0 * f(x4));
    let mid = 0.5 * (a + b);
    let left = super::quadrature::simpson(f, a, mid, 4);
    let right = super::quadrature::simpson(f, mid, b, 4);
    let split = left + right;
    if depth >= max_depth || (split - whole).abs() < 15.0 * tol {
        return split + (split - whole) / 15.0;
    }
    adaptive_boole_rec(f, a, mid, tol * 0.5, max_depth, depth + 1)
        + adaptive_boole_rec(f, mid, b, tol * 0.5, max_depth, depth + 1)
}

pub fn double_exponential(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let pi_half = std::f64::consts::FRAC_PI_2;
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    let h = 6.0 / n as f64;
    let mut sum = 0.0;
    for k in 0..=n {
        let t = -3.0 + k as f64 * h;
        let phi = pi_half * t.sinh();
        let phi_prime = pi_half * t.cosh();
        let x_t = phi.tanh();
        let cosh_phi = phi.cosh();
        let w = phi_prime / (cosh_phi * cosh_phi);
        if w.abs() < 1e-30 {
            continue;
        }
        let xm = mid + half * x_t;
        if xm > a && xm < b {
            sum += w * f(xm);
        }
    }
    sum * half * h
}

pub fn cauchy_principal_value(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    singularity: f64,
    epsilon: f64,
    n: usize,
) -> f64 {
    let left = if singularity - epsilon > a {
        super::quadrature::simpson(&f, a, singularity - epsilon, n)
    } else {
        0.0
    };
    let right = if singularity + epsilon < b {
        super::quadrature::simpson(&f, singularity + epsilon, b, n)
    } else {
        0.0
    };
    left + right
}

pub fn improper_both_infinite(f: impl Fn(f64) -> f64, n: usize) -> f64 {
    let g = |t: f64| {
        let x = t / (1.0 - t * t);
        let dx = (1.0 + t * t) / ((1.0 - t * t) * (1.0 - t * t));
        f(x) * dx
    };
    super::quadrature::simpson(g, -1.0 + 1e-10, 1.0 - 1e-10, n)
}

pub fn improper_left_infinite(f: impl Fn(f64) -> f64, b: f64, n: usize) -> f64 {
    let g = |t: f64| {
        if t.abs() < 1e-30 {
            return 0.0;
        }
        let x = b - (1.0 - t) / t;
        f(x) / (t * t)
    };
    super::quadrature::simpson(g, 1e-10, 1.0, n)
}

pub fn numerical_derivative_central(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}

pub fn numerical_second_derivative(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h)
}

pub fn numerical_derivative_5point(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (-f(x + 2.0 * h) + 8.0 * f(x + h) - 8.0 * f(x - h) + f(x - 2.0 * h)) / (12.0 * h)
}

pub fn numerical_integral_cumulative(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
) -> Vec<(f64, f64)> {
    let h = (b - a) / n as f64;
    let mut result = Vec::with_capacity(n + 1);
    result.push((a, 0.0));
    let mut cumul = 0.0;
    for i in 1..=n {
        let x0 = a + (i - 1) as f64 * h;
        let x1 = a + i as f64 * h;
        cumul += 0.5 * h * (f(x0) + f(x1));
        result.push((x1, cumul));
    }
    result
}
