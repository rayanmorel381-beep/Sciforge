pub fn monod_growth(mu_max: f64, s: f64, ks: f64) -> f64 {
    mu_max * s / (ks + s)
}

pub fn monod_dynamics(
    x: f64,
    s: f64,
    mu_max: f64,
    ks: f64,
    y: f64,
    d: f64,
    s_in: f64,
) -> (f64, f64) {
    let mu = monod_growth(mu_max, s, ks);
    let dx = (mu - d) * x;
    let ds = d * (s_in - s) - mu * x / y;
    (dx, ds)
}

pub fn monod_simulate(
    x0: f64,
    s0: f64,
    mu_max: f64,
    ks: f64,
    y: f64,
    d: f64,
    s_in: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut x, mut s) = (x0, s0);
    result.push((x, s));
    for _ in 0..steps {
        let (dx, ds) = monod_dynamics(x, s, mu_max, ks, y, d, s_in);
        x = (x + dx * dt).max(0.0);
        s = (s + ds * dt).max(0.0);
        result.push((x, s));
    }
    result
}

pub fn bacterial_growth_phases(n0: f64, mu: f64, k: f64, lag: f64, t: f64) -> f64 {
    if t < lag {
        n0
    } else {
        let t_eff = t - lag;
        k / (1.0 + ((k - n0) / n0) * (-mu * t_eff).exp())
    }
}

pub fn generation_time_bacteria(mu: f64) -> f64 {
    (2.0_f64).ln() / mu
}

pub fn death_phase(n_peak: f64, death_rate: f64, t: f64) -> f64 {
    n_peak * (-death_rate * t).exp()
}

pub fn diauxic_growth(s1: f64, s2: f64, mu1: f64, mu2: f64, ks1: f64, ks2: f64, ki: f64) -> f64 {
    let mu_s1 = mu1 * s1 / (ks1 + s1);
    let mu_s2 = mu2 * s2 / (ks2 + s2) / (1.0 + s1 / ki);
    mu_s1 + mu_s2
}
