use crate::constants::{
    GAUSS_7_WEIGHTS, GAUSS_HERMITE_5_NODES, GAUSS_HERMITE_5_WEIGHTS, GAUSS_KRONROD_15_NODES,
    GAUSS_KRONROD_15_WEIGHTS, GAUSS_LAGUERRE_5_NODES, GAUSS_LAGUERRE_5_WEIGHTS,
    GAUSS_LEGENDRE_5_NODES, GAUSS_LEGENDRE_5_WEIGHTS, LOBATTO_5_NODES, LOBATTO_5_WEIGHTS,
};

pub fn trapezoid(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    sum * h
}

pub fn simpson(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let n = if n % 2 == 1 { n + 1 } else { n };
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += if i % 2 == 0 { 2.0 } else { 4.0 } * f(x);
    }
    sum * h / 3.0
}

pub fn simpson_38(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let n = n - n % 3;
    let n = if n == 0 { 3 } else { n };
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += if i % 3 == 0 { 2.0 } else { 3.0 } * f(x);
    }
    sum * 3.0 * h / 8.0
}

pub fn boole(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let n = n - n % 4;
    let n = if n == 0 { 4 } else { n };
    let h = (b - a) / n as f64;
    let mut sum = 0.0;
    for i in (0..n).step_by(4) {
        let x0 = a + i as f64 * h;
        sum += 7.0 * f(x0)
            + 32.0 * f(x0 + h)
            + 12.0 * f(x0 + 2.0 * h)
            + 32.0 * f(x0 + 3.0 * h)
            + 7.0 * f(x0 + 4.0 * h);
    }
    sum * 2.0 * h / 45.0
}

pub fn midpoint(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;
    for i in 0..n {
        sum += f(a + (i as f64 + 0.5) * h);
    }
    sum * h
}

pub fn romberg(f: impl Fn(f64) -> f64, a: f64, b: f64, max_order: usize) -> f64 {
    let mut r = vec![vec![0.0; max_order]; max_order];
    r[0][0] = 0.5 * (b - a) * (f(a) + f(b));

    for i in 1..max_order {
        let n = 1u64 << i;
        let h = (b - a) / n as f64;
        let mut sum = 0.0;
        for k in 0..n {
            let x = a + (k as f64 + 0.5) * h;
            sum += f(x);
        }
        r[i][0] = 0.5 * r[i - 1][0] + sum * h;

        for j in 1..=i {
            let factor = 4.0_f64.powi(j as i32);
            r[i][j] = (factor * r[i][j - 1] - r[i - 1][j - 1]) / (factor - 1.0);
        }
    }
    r[max_order - 1][max_order - 1]
}

pub fn gauss_legendre_5(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let nodes = GAUSS_LEGENDRE_5_NODES;
    let weights = GAUSS_LEGENDRE_5_WEIGHTS;
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    let mut sum = 0.0;
    for i in 0..5 {
        sum += weights[i] * f(mid + half * nodes[i]);
    }
    sum * half
}

pub fn gauss_laguerre_5(f: impl Fn(f64) -> f64) -> f64 {
    let nodes = GAUSS_LAGUERRE_5_NODES;
    let weights = GAUSS_LAGUERRE_5_WEIGHTS;
    let mut sum = 0.0;
    for i in 0..5 {
        sum += weights[i] * f(nodes[i]);
    }
    sum
}

pub fn gauss_hermite_5(f: impl Fn(f64) -> f64) -> f64 {
    let nodes = GAUSS_HERMITE_5_NODES;
    let weights = GAUSS_HERMITE_5_WEIGHTS;
    let mut sum = 0.0;
    for i in 0..5 {
        sum += weights[i] * f(nodes[i]);
    }
    sum
}

pub fn gauss_chebyshev_5(f: impl Fn(f64) -> f64) -> f64 {
    let pi = std::f64::consts::PI;
    let n = 5;
    let w = pi / n as f64;
    let mut sum = 0.0;
    for k in 1..=n {
        let xk = ((2.0 * k as f64 - 1.0) * pi / (2.0 * n as f64)).cos();
        sum += f(xk);
    }
    sum * w
}

pub fn composite_simpson(f: impl Fn(f64) -> f64, a: f64, b: f64, panels: usize) -> f64 {
    let panels = if panels % 2 == 1 { panels + 1 } else { panels };
    let h = (b - a) / panels as f64;
    let mut sum = f(a) + f(b);
    for i in 1..panels {
        let x = a + i as f64 * h;
        sum += if i % 2 == 0 { 2.0 } else { 4.0 } * f(x);
    }
    sum * h / 3.0
}

pub fn composite_trapezoid(f: impl Fn(f64) -> f64, a: f64, b: f64, panels: usize) -> f64 {
    let h = (b - a) / panels as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..panels {
        sum += f(a + i as f64 * h);
    }
    sum * h
}

pub fn weddle(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let n = n - n % 6;
    let n = if n == 0 { 6 } else { n };
    let h = (b - a) / n as f64;
    let mut sum = 0.0;
    for i in (0..n).step_by(6) {
        let x = a + i as f64 * h;
        sum += f(x)
            + 5.0 * f(x + h)
            + f(x + 2.0 * h)
            + 6.0 * f(x + 3.0 * h)
            + f(x + 4.0 * h)
            + 5.0 * f(x + 5.0 * h)
            + f(x + 6.0 * h);
    }
    sum * 3.0 * h / 10.0
}

pub fn clenshaw_curtis(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let pi = std::f64::consts::PI;
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    let mut sum = 0.0;
    for k in 0..=n {
        let theta = pi * k as f64 / n as f64;
        let x = mid + half * theta.cos();
        let w = if k == 0 || k == n {
            1.0 / (n * n - 1) as f64
        } else {
            let mut wk = 0.0;
            for j in 1..=(n / 2) {
                let bj = if 2 * j == n { 1.0 } else { 2.0 };
                wk += bj * (2.0 * j as f64 * theta).cos() / (4.0 * j as f64 * j as f64 - 1.0);
            }
            2.0 / n as f64 * (1.0 - wk)
        };
        sum += w * f(x);
    }
    sum * half
}

pub fn tanh_sinh(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let pi_half = std::f64::consts::FRAC_PI_2;
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    let h = 4.0 / n as f64;
    let mut sum = 0.0;
    for k in -(n as i64)..=(n as i64) {
        let t = k as f64 * h;
        let sinh_t = t.sinh();
        let u = pi_half * sinh_t;
        let cosh_u = u.cosh();
        if cosh_u.abs() < 1e-30 {
            continue;
        }
        let x = u.tanh();
        let w = pi_half * t.cosh() / (cosh_u * cosh_u);
        if w.abs() < 1e-30 {
            continue;
        }
        let xm = mid + half * x;
        if xm > a && xm < b {
            sum += w * f(xm);
        }
    }
    sum * half * h
}

pub fn lobatto_quadrature(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let nodes = LOBATTO_5_NODES;
    let weights = LOBATTO_5_WEIGHTS;
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    let mut sum = 0.0;
    for i in 0..5 {
        sum += weights[i] * f(mid + half * nodes[i]);
    }
    sum * half
}

pub fn open_newton_cotes_4(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let h = (b - a) / 6.0;
    (b - a) / 24.0
        * (11.0 * f(a + h) - 14.0 * f(a + 2.0 * h) + 26.0 * f(a + 3.0 * h) - 14.0 * f(a + 4.0 * h)
            + 11.0 * f(a + 5.0 * h))
}

pub fn gauss_kronrod_15(f: impl Fn(f64) -> f64, a: f64, b: f64) -> (f64, f64) {
    let nodes_k = GAUSS_KRONROD_15_NODES;
    let weights_k = GAUSS_KRONROD_15_WEIGHTS;
    let weights_g = GAUSS_7_WEIGHTS;
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    let mut result_k = 0.0;
    let mut result_g = 0.0;
    for i in 0..15 {
        let x = mid + half * nodes_k[i];
        let fv = f(x);
        result_k += weights_k[i] * fv;
        if i % 2 == 1 {
            result_g += weights_g[i / 2] * fv;
        }
    }
    (result_k * half, (result_k - result_g).abs() * half)
}

pub fn richardson_extrapolation(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
    order: usize,
) -> f64 {
    let mut t = Vec::with_capacity(order);
    for k in 0..order {
        let panels = n * (1 << k);
        t.push(trapezoid(&f, a, b, panels));
    }
    for j in 1..order {
        for i in (j..order).rev() {
            let factor = 4.0_f64.powi(j as i32);
            t[i] = (factor * t[i] - t[i - 1]) / (factor - 1.0);
        }
    }
    t[order - 1]
}

pub fn filon_sin(f: impl Fn(f64) -> f64, a: f64, b: f64, omega: f64, n: usize) -> f64 {
    let n = if n % 2 == 1 { n + 1 } else { n };
    let h = (b - a) / n as f64;
    let theta = omega * h;
    let (alpha, beta, gamma) = if theta.abs() < 1e-6 {
        (2.0 * theta / 3.0, 2.0 / 3.0, 4.0 / 3.0)
    } else {
        let s = theta.sin();
        let c = theta.cos();
        let t2 = theta * theta;
        let a_ = (theta + theta * c * c - 2.0 * s * c) / t2;
        let b_ = 2.0 * (theta * (1.0 + c * c) - 2.0 * s * c) / t2;
        let g_ = 4.0 * (s - theta * c) / t2;
        (a_, b_, g_)
    };
    let mut c_even = 0.0;
    let mut c_odd = 0.0;
    for i in 0..=n {
        let x = a + i as f64 * h;
        let val = f(x) * (omega * x).sin();
        if i == 0 || i == n {
            continue;
        } else if i % 2 == 0 {
            c_even += val;
        } else {
            c_odd += val;
        }
    }
    h * (alpha * (f(a) * (omega * a).cos() - f(b) * (omega * b).cos())
        + beta * c_even
        + gamma * c_odd)
}

pub fn filon_cos(f: impl Fn(f64) -> f64, a: f64, b: f64, omega: f64, n: usize) -> f64 {
    let n = if n % 2 == 1 { n + 1 } else { n };
    let h = (b - a) / n as f64;
    let theta = omega * h;
    let (alpha, beta, gamma) = if theta.abs() < 1e-6 {
        (2.0 * theta / 3.0, 2.0 / 3.0, 4.0 / 3.0)
    } else {
        let s = theta.sin();
        let c = theta.cos();
        let t2 = theta * theta;
        let a_ = (theta + theta * c * c - 2.0 * s * c) / t2;
        let b_ = 2.0 * (theta * (1.0 + c * c) - 2.0 * s * c) / t2;
        let g_ = 4.0 * (s - theta * c) / t2;
        (a_, b_, g_)
    };
    let mut c_even = 0.0;
    let mut c_odd = 0.0;
    for i in 0..=n {
        let x = a + i as f64 * h;
        let val = f(x) * (omega * x).cos();
        if i == 0 || i == n {
            continue;
        } else if i % 2 == 0 {
            c_even += val;
        } else {
            c_odd += val;
        }
    }
    h * (alpha * (f(a) * (omega * a).sin() - f(b) * (omega * b).sin())
        + beta * c_even
        + gamma * c_odd)
}
