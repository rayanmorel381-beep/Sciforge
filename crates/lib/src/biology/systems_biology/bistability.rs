pub fn toggle_switch(
    u: f64,
    v: f64,
    alpha1: f64,
    alpha2: f64,
    beta: f64,
    gamma: f64,
    n: f64,
) -> (f64, f64) {
    let du = alpha1 / (1.0 + v.powf(n)) - beta * u;
    let dv = alpha2 / (1.0 + u.powf(n)) - gamma * v;
    (du, dv)
}

pub fn bistable_toggle_switch(
    u0: f64,
    v0: f64,
    alpha1: f64,
    alpha2: f64,
    beta: f64,
    gamma: f64,
    n: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut u, mut v) = (u0, v0);
    result.push((u, v));
    for _ in 0..steps {
        let (du, dv) = toggle_switch(u, v, alpha1, alpha2, beta, gamma, n);
        u = (u + du * dt).max(0.0);
        v = (v + dv * dt).max(0.0);
        result.push((u, v));
    }
    result
}

pub fn ultrasensitivity_index(ec10: f64, ec90: f64) -> f64 {
    if ec10 < 1e-30 {
        return 0.0;
    }
    (ec90 / ec10).log10() / (81.0_f64).log10()
}

pub fn network_robustness(
    output_nominal: f64,
    output_perturbed: f64,
    perturbation_fraction: f64,
) -> f64 {
    if output_nominal.abs() < 1e-30 || perturbation_fraction.abs() < 1e-30 {
        return 0.0;
    }
    let relative_change = (output_perturbed - output_nominal).abs() / output_nominal.abs();
    1.0 - relative_change / perturbation_fraction
}

pub fn adaptation_precision(response_peak: f64, response_steady: f64, response_basal: f64) -> f64 {
    if (response_peak - response_basal).abs() < 1e-30 {
        return 0.0;
    }
    1.0 - (response_steady - response_basal).abs() / (response_peak - response_basal).abs()
}

pub fn bifurcation_parameter_scan(
    f: fn(f64, f64) -> f64,
    x0: f64,
    param_range: &[f64],
    dt: f64,
    settle: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(param_range.len());
    for &p in param_range {
        let mut x = x0;
        for _ in 0..settle {
            x += f(x, p) * dt;
        }
        result.push((p, x));
    }
    result
}

pub fn hysteresis_width(forward_thresholds: &[f64], backward_thresholds: &[f64]) -> f64 {
    let fwd = forward_thresholds
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let bwd = backward_thresholds
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    (fwd - bwd).abs()
}

pub fn nullcline_intersection(
    f: fn(f64, f64) -> f64,
    g: fn(f64, f64) -> f64,
    x_range: (f64, f64),
    y_range: (f64, f64),
    resolution: usize,
) -> Vec<(f64, f64)> {
    let dx = (x_range.1 - x_range.0) / resolution as f64;
    let dy = (y_range.1 - y_range.0) / resolution as f64;
    let mut intersections = Vec::new();
    for i in 0..resolution {
        for j in 0..resolution {
            let x = x_range.0 + i as f64 * dx;
            let y = y_range.0 + j as f64 * dy;
            let fv = f(x, y);
            let gv = g(x, y);
            if fv.abs() < dx * 10.0 && gv.abs() < dy * 10.0 {
                intersections.push((x, y));
            }
        }
    }
    intersections
}

pub fn saddle_node_condition(jacobian: &[[f64; 2]; 2]) -> f64 {
    jacobian[0][0] * jacobian[1][1] - jacobian[0][1] * jacobian[1][0]
}

pub fn potential_landscape_1d(
    f: fn(f64) -> f64,
    x_range: (f64, f64),
    n_points: usize,
) -> Vec<(f64, f64)> {
    let dx = (x_range.1 - x_range.0) / n_points as f64;
    let mut potential = 0.0;
    let mut result = Vec::with_capacity(n_points);
    for i in 0..n_points {
        let x = x_range.0 + i as f64 * dx;
        potential -= f(x) * dx;
        result.push((x, potential));
    }
    result
}
