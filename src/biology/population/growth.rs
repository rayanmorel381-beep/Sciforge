pub fn logistic_growth(n: f64, r: f64, k: f64) -> f64 {
    r * n * (1.0 - n / k)
}

pub fn logistic_solve(n0: f64, r: f64, k: f64, dt: f64, steps: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut n = n0;
    result.push(n);
    for _ in 0..steps {
        let dn = logistic_growth(n, r, k);
        n += dn * dt;
        n = n.max(0.0);
        result.push(n);
    }
    result
}

pub fn exponential_growth(n0: f64, r: f64, t: f64) -> f64 {
    n0 * (r * t).exp()
}

pub fn gompertz(n: f64, a: f64, b: f64, k: f64) -> f64 {
    a * n * (k / n).ln().max(0.0) * (-b).exp()
}

pub fn allee_effect(n: f64, r: f64, k: f64, a: f64) -> f64 {
    r * n * (n / a - 1.0) * (1.0 - n / k)
}

pub fn beverton_holt(n: f64, r: f64, k: f64) -> f64 {
    r * n / (1.0 + (r - 1.0) * n / k)
}

pub fn ricker(n: f64, r: f64, k: f64) -> f64 {
    n * (r * (1.0 - n / k)).exp()
}

pub fn doubling_time(r: f64) -> f64 {
    (2.0_f64).ln() / r
}

pub fn von_bertalanffy(l_inf: f64, k: f64, t: f64, t0: f64) -> f64 {
    l_inf * (1.0 - (-k * (t - t0)).exp())
}

pub fn theta_logistic(n: f64, r: f64, k: f64, theta: f64) -> f64 {
    r * n * (1.0 - (n / k).powf(theta))
}

pub fn moran_process_fixation(n: usize, r: f64) -> f64 {
    if (r - 1.0).abs() < 1e-12 {
        return 1.0 / n as f64;
    }
    (1.0 - 1.0 / r) / (1.0 - (1.0 / r).powi(n as i32))
}
