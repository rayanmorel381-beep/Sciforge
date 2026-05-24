pub fn lagrange_interpolate(points: &[(f64, f64)], x: f64) -> f64 {
    let n = points.len();
    let mut result = 0.0;
    for i in 0..n {
        let mut basis = 1.0;
        for j in 0..n {
            if i != j {
                basis *= (x - points[j].0) / (points[i].0 - points[j].0);
            }
        }
        result += basis * points[i].1;
    }
    result
}

pub fn lagrange_barycentric(points: &[(f64, f64)], x: f64) -> f64 {
    let n = points.len();
    let mut weights = vec![1.0; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                weights[i] /= points[i].0 - points[j].0;
            }
        }
    }

    let mut numer = 0.0;
    let mut denom = 0.0;
    for i in 0..n {
        let diff = x - points[i].0;
        if diff.abs() < 1e-30 {
            return points[i].1;
        }
        let t = weights[i] / diff;
        numer += t * points[i].1;
        denom += t;
    }
    numer / denom
}

pub fn neville(points: &[(f64, f64)], x: f64) -> f64 {
    let n = points.len();
    let mut table: Vec<f64> = points.iter().map(|p| p.1).collect();
    for k in 1..n {
        for i in 0..n - k {
            table[i] = ((x - points[i + k].0) * table[i] - (x - points[i].0) * table[i + 1])
                / (points[i].0 - points[i + k].0);
        }
    }
    table[0]
}

pub fn divided_differences(points: &[(f64, f64)]) -> Vec<f64> {
    let n = points.len();
    let mut dd: Vec<f64> = points.iter().map(|p| p.1).collect();
    for k in 1..n {
        for i in (k..n).rev() {
            dd[i] = (dd[i] - dd[i - 1]) / (points[i].0 - points[i - k].0);
        }
    }
    dd
}

pub fn newton_interpolate(points: &[(f64, f64)], x: f64) -> f64 {
    let dd = divided_differences(points);
    let mut result = dd[points.len() - 1];
    for i in (0..points.len() - 1).rev() {
        result = result * (x - points[i].0) + dd[i];
    }
    result
}

pub fn lagrange_derivative(points: &[(f64, f64)], x: f64) -> f64 {
    let n = points.len();
    let mut result = 0.0;
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            if j == i {
                continue;
            }
            let mut prod = 1.0;
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }
                prod *= (x - points[k].0) / (points[i].0 - points[k].0);
            }
            sum += prod / (points[i].0 - points[j].0);
        }
        result += points[i].1 * sum;
    }
    result
}

pub fn chebyshev_nodes(n: usize, a: f64, b: f64) -> Vec<f64> {
    let pi = std::f64::consts::PI;
    (0..n)
        .map(|k| {
            let t = ((2 * k + 1) as f64 * pi / (2.0 * n as f64)).cos();
            0.5 * (a + b) + 0.5 * (b - a) * t
        })
        .collect()
}

pub fn chebyshev_interpolate(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize, x: f64) -> f64 {
    let nodes = chebyshev_nodes(n, a, b);
    let points: Vec<(f64, f64)> = nodes.iter().map(|&xi| (xi, f(xi))).collect();
    lagrange_barycentric(&points, x)
}

pub fn rational_interpolate(points: &[(f64, f64)], x: f64) -> f64 {
    let n = points.len();
    let mut c: Vec<f64> = points.iter().map(|p| p.1).collect();
    let mut d: Vec<f64> = c.clone();
    let mut ns = 0;
    let mut dif = (x - points[0].0).abs();
    for (i, pi) in points.iter().enumerate().skip(1) {
        let dift = (x - pi.0).abs();
        if dift < dif {
            ns = i;
            dif = dift;
        }
    }
    let mut y = points[ns].1;
    ns -= 0;
    for m in 1..n {
        for i in 0..n - m {
            let w = c[i + 1] - d[i];
            let h = points[i + m].0 - x;
            let t = (points[i].0 - x) * d[i] / h;
            let dd = t - c[i + 1];
            if dd.abs() < 1e-30 {
                return y;
            }
            let dd = w / dd;
            d[i] = c[i + 1] * dd;
            c[i] = t * dd;
        }
        if 2 * (ns + 1) < n - m {
            y += c[ns + 1];
        } else {
            y += d[ns];
            ns = ns.saturating_sub(1);
        }
    }
    y
}

pub fn newton_forward_difference(points: &[(f64, f64)], x: f64) -> f64 {
    let n = points.len();
    let h = if n > 1 {
        points[1].0 - points[0].0
    } else {
        1.0
    };
    let s = (x - points[0].0) / h;
    let mut table: Vec<Vec<f64>> = vec![points.iter().map(|p| p.1).collect()];
    for k in 1..n {
        let prev = &table[k - 1];
        let diff: Vec<f64> = (0..prev.len() - 1).map(|i| prev[i + 1] - prev[i]).collect();
        table.push(diff);
    }
    let mut result = table[0][0];
    let mut s_prod = 1.0;
    for (k, tk) in table.iter().enumerate().skip(1) {
        s_prod *= (s - (k - 1) as f64) / k as f64;
        if tk.is_empty() {
            break;
        }
        result += s_prod * tk[0];
    }
    result
}

pub fn interpolation_error_bound(points: &[(f64, f64)], x: f64, max_deriv: f64) -> f64 {
    let n = points.len();
    let mut prod = 1.0;
    for pi in points {
        prod *= (x - pi.0).abs();
    }
    let mut factorial = 1.0;
    for i in 1..=n {
        factorial *= i as f64;
    }
    max_deriv * prod / factorial
}

pub fn lebesgue_constant(nodes: &[f64], eval_points: &[f64]) -> f64 {
    let n = nodes.len();
    let mut max_lambda = 0.0_f64;
    for &x in eval_points {
        let mut lambda = 0.0;
        for i in 0..n {
            let mut li = 1.0;
            for j in 0..n {
                if i != j {
                    li *= (x - nodes[j]) / (nodes[i] - nodes[j]);
                }
            }
            lambda += li.abs();
        }
        max_lambda = max_lambda.max(lambda);
    }
    max_lambda
}
