pub fn boolean_network_step(state: &[bool], rules: &[Vec<(usize, bool)>]) -> Vec<bool> {
    let n = state.len();
    let mut next = vec![false; n];
    for i in 0..n {
        if i < rules.len() {
            let mut active = false;
            for &(input_idx, polarity) in &rules[i] {
                if input_idx < n && state[input_idx] == polarity {
                    active = true;
                    break;
                }
            }
            next[i] = active;
        }
    }
    next
}

pub fn boolean_network_simulate(
    initial: &[bool],
    rules: &[Vec<(usize, bool)>],
    steps: usize,
) -> Vec<Vec<bool>> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut state = initial.to_vec();
    result.push(state.clone());
    for _ in 0..steps {
        state = boolean_network_step(&state, rules);
        result.push(state.clone());
    }
    result
}

pub fn gene_regulatory_hill(
    activators: &[(f64, f64, f64)],
    repressors: &[(f64, f64, f64)],
    basal: f64,
) -> f64 {
    let mut activation = basal;
    for &(conc, k, n) in activators {
        let cn = conc.powf(n);
        activation += cn / (k.powf(n) + cn);
    }
    for &(conc, k, n) in repressors {
        let cn = conc.powf(n);
        activation *= k.powf(n) / (k.powf(n) + cn);
    }
    activation
}

pub fn repressilator(
    m: &[f64; 3],
    p: &[f64; 3],
    alpha: f64,
    alpha0: f64,
    beta: f64,
    n: f64,
) -> ([f64; 3], [f64; 3]) {
    let mut dm = [0.0; 3];
    let mut dp = [0.0; 3];
    for i in 0..3 {
        let repressor = p[(i + 2) % 3];
        let rn = repressor.powf(n);
        dm[i] = -m[i] + alpha / (1.0 + rn) + alpha0;
        dp[i] = -beta * (p[i] - m[i]);
    }
    (dm, dp)
}

pub fn repressilator_simulate(
    m0: [f64; 3],
    p0: [f64; 3],
    alpha: f64,
    alpha0: f64,
    beta: f64,
    n: f64,
    dt: f64,
    steps: usize,
) -> Vec<([f64; 3], [f64; 3])> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut m = m0;
    let mut p = p0;
    result.push((m, p));
    for _ in 0..steps {
        let (dm, dp) = repressilator(&m, &p, alpha, alpha0, beta, n);
        for i in 0..3 {
            m[i] = (m[i] + dm[i] * dt).max(0.0);
            p[i] = (p[i] + dp[i] * dt).max(0.0);
        }
        result.push((m, p));
    }
    result
}

pub fn genetic_toggle_switch(
    u: f64,
    v: f64,
    alpha1: f64,
    alpha2: f64,
    beta: f64,
    gamma: f64,
) -> (f64, f64) {
    let du = alpha1 / (1.0 + v.powf(beta)) - u;
    let dv = alpha2 / (1.0 + u.powf(gamma)) - v;
    (du, dv)
}

pub fn toggle_switch_simulate(
    u0: f64,
    v0: f64,
    alpha1: f64,
    alpha2: f64,
    beta: f64,
    gamma: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut u = u0;
    let mut v = v0;
    result.push((u, v));
    for _ in 0..steps {
        let (du, dv) = genetic_toggle_switch(u, v, alpha1, alpha2, beta, gamma);
        u = (u + du * dt).max(0.0);
        v = (v + dv * dt).max(0.0);
        result.push((u, v));
    }
    result
}

pub fn feed_forward_loop(input: f64, x: f64, k_xy: f64, k_iy: f64, n: f64, coherent: bool) -> f64 {
    let x_contrib = x.powf(n) / (k_xy.powf(n) + x.powf(n));
    let i_contrib = input.powf(n) / (k_iy.powf(n) + input.powf(n));
    if coherent {
        x_contrib * i_contrib
    } else {
        x_contrib * (1.0 - i_contrib)
    }
}

pub fn sensitivity_analysis(f: impl Fn(f64) -> f64, param: f64, delta: f64) -> f64 {
    let y0 = f(param);
    let y1 = f(param + delta);
    if y0.abs() < 1e-30 {
        return (y1 - y0) / delta;
    }
    ((y1 - y0) / delta) * (param / y0)
}

pub fn robustness_coefficient(nominal_output: f64, perturbed_outputs: &[f64]) -> f64 {
    if perturbed_outputs.is_empty() {
        return 1.0;
    }
    let mean_deviation = perturbed_outputs
        .iter()
        .map(|&p| ((p - nominal_output) / nominal_output.max(1e-30)).abs())
        .sum::<f64>()
        / perturbed_outputs.len() as f64;
    1.0 / (1.0 + mean_deviation)
}
