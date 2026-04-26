pub fn waddington_landscape(x: f64, param: f64) -> f64 {
    x.powi(4) - param * x.powi(2)
}

pub fn differentiation_potential(x: f64, param: f64) -> f64 {
    4.0 * x.powi(3) - 2.0 * param * x
}

pub fn differentiation_simulate(
    x0: f64,
    param_start: f64,
    param_end: f64,
    noise: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut x = x0;
    result.push((param_start, x));
    for step in 0..steps {
        let t_frac = step as f64 / steps as f64;
        let param = param_start + (param_end - param_start) * t_frac;
        let force = -differentiation_potential(x, param);
        let pseudo_noise = noise * ((step as f64 * 0.618).sin() * 2.0 - 1.0);
        x += (force + pseudo_noise) * dt;
        result.push((param, x));
    }
    result
}

pub fn somitogenesis_clock(phase: f64, omega: f64, coupling: f64, neighbor_phase: f64) -> f64 {
    omega + coupling * (neighbor_phase - phase).sin()
}

pub fn somite_clock_simulate(
    phases: &mut [f64],
    omega: f64,
    coupling: f64,
    dt: f64,
    steps: usize,
) -> Vec<Vec<f64>> {
    let n = phases.len();
    let mut result = Vec::with_capacity(steps + 1);
    result.push(phases.to_vec());
    for _ in 0..steps {
        let old = phases.to_vec();
        for i in 0..n {
            let left = if i > 0 { old[i - 1] } else { old[i] };
            let right = if i < n - 1 { old[i + 1] } else { old[i] };
            let avg_neighbor = (left + right) / 2.0;
            let dphase = somitogenesis_clock(old[i], omega, coupling, avg_neighbor);
            phases[i] = old[i] + dphase * dt;
        }
        result.push(phases.to_vec());
    }
    result
}

pub fn lateral_inhibition_step(
    signal: f64,
    neighbor_signal: f64,
    delta: f64,
    notch: f64,
    k: f64,
) -> (f64, f64) {
    let ddelta = -delta + signal;
    let dnotch = -notch + k * neighbor_signal;
    (ddelta, dnotch)
}

pub fn cell_fate_probability(signal: f64, threshold: f64, steepness: f64) -> f64 {
    1.0 / (1.0 + (-(signal - threshold) * steepness).exp())
}
