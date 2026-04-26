pub fn goodwin_oscillator_step(
    mrna: f64,
    protein: f64,
    repressor: f64,
    dt: f64,
    k1: f64,
    k2: f64,
    k3: f64,
    d1: f64,
    d2: f64,
    d3: f64,
    n: f64,
    ki: f64,
) -> (f64, f64, f64) {
    let dm = k1 / (1.0 + (repressor / ki).powf(n)) - d1 * mrna;
    let dp = k2 * mrna - d2 * protein;
    let dr = k3 * protein - d3 * repressor;
    (
        (mrna + dm * dt).max(0.0),
        (protein + dp * dt).max(0.0),
        (repressor + dr * dt).max(0.0),
    )
}

pub fn repressilator_step(
    x: &[f64; 3],
    dt: f64,
    alpha: f64,
    alpha0: f64,
    beta: f64,
    n: f64,
) -> [f64; 3] {
    let mut dx = [0.0; 3];
    for i in 0..3 {
        let repressor = x[(i + 2) % 3];
        dx[i] = -x[i] + alpha / (1.0 + repressor.powf(n)) + alpha0;
    }
    [
        (x[0] + dx[0] * dt * beta).max(0.0),
        (x[1] + dx[1] * dt * beta).max(0.0),
        (x[2] + dx[2] * dt * beta).max(0.0),
    ]
}

pub fn oscillation_period(time_series: &[f64], dt: f64) -> f64 {
    if time_series.len() < 3 {
        return 0.0;
    }
    let mean: f64 = time_series.iter().sum::<f64>() / time_series.len() as f64;
    let mut crossings = Vec::new();
    for i in 1..time_series.len() {
        if (time_series[i - 1] - mean) * (time_series[i] - mean) < 0.0 {
            crossings.push(i as f64 * dt);
        }
    }
    if crossings.len() < 2 {
        return 0.0;
    }
    let mut total = 0.0;
    for i in 1..crossings.len() {
        total += crossings[i] - crossings[i - 1];
    }
    2.0 * total / (crossings.len() - 1) as f64
}

pub fn oscillation_amplitude(time_series: &[f64]) -> f64 {
    if time_series.is_empty() {
        return 0.0;
    }
    let max = time_series
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let min = time_series.iter().cloned().fold(f64::INFINITY, f64::min);
    (max - min) / 2.0
}

pub fn phase_plane_nullcline_x(y: f64, params: (f64, f64, f64)) -> f64 {
    let (a, b, _) = params;
    a * y / (b + y)
}

pub fn fitzhugh_nagumo_step(
    v: f64,
    w: f64,
    dt: f64,
    i_ext: f64,
    a: f64,
    b: f64,
    tau: f64,
) -> (f64, f64) {
    let dv = v - v.powi(3) / 3.0 - w + i_ext;
    let dw = (v + a - b * w) / tau;
    (v + dv * dt, w + dw * dt)
}

pub fn entrainment_arnold_tongue(coupling_strength: f64, frequency_mismatch: f64) -> bool {
    coupling_strength > frequency_mismatch.abs()
}
