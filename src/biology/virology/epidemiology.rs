pub fn sir_step(s: f64, i: f64, r: f64, beta: f64, gamma: f64, dt: f64) -> (f64, f64, f64) {
    let n = s + i + r;
    let ds = -beta * s * i / n;
    let di = beta * s * i / n - gamma * i;
    let dr = gamma * i;
    (
        (s + ds * dt).max(0.0),
        (i + di * dt).max(0.0),
        (r + dr * dt).max(0.0),
    )
}

pub fn seir_step(
    s: f64,
    e: f64,
    i: f64,
    r: f64,
    beta: f64,
    sigma: f64,
    gamma: f64,
    dt: f64,
) -> (f64, f64, f64, f64) {
    let n = s + e + i + r;
    let ds = -beta * s * i / n;
    let de = beta * s * i / n - sigma * e;
    let di = sigma * e - gamma * i;
    let dr = gamma * i;
    (
        (s + ds * dt).max(0.0),
        (e + de * dt).max(0.0),
        (i + di * dt).max(0.0),
        (r + dr * dt).max(0.0),
    )
}

pub fn basic_reproduction_number(beta: f64, gamma: f64) -> f64 {
    if gamma.abs() < 1e-15 {
        return 0.0;
    }
    beta / gamma
}

pub fn herd_immunity_threshold(r0: f64) -> f64 {
    if r0 <= 1.0 {
        return 0.0;
    }
    1.0 - 1.0 / r0
}

pub fn serial_interval_mean(incubation_mean: f64, infectious_offset: f64) -> f64 {
    incubation_mean + infectious_offset
}

pub fn rt_effective(r0: f64, fraction_susceptible: f64) -> f64 {
    r0 * fraction_susceptible
}

pub fn epidemic_growth_rate(r0: f64, generation_time: f64) -> f64 {
    if generation_time.abs() < 1e-15 {
        return 0.0;
    }
    (r0 - 1.0) / generation_time
}

pub fn doubling_time(growth_rate: f64) -> f64 {
    if growth_rate <= 0.0 {
        return f64::INFINITY;
    }
    std::f64::consts::LN_2 / growth_rate
}

pub fn attack_rate(r0: f64) -> f64 {
    let mut ar = 0.99;
    for _ in 0..200 {
        ar = 1.0 - (-r0 * ar).exp();
    }
    ar
}

pub fn case_fatality_rate(deaths: f64, cases: f64) -> f64 {
    if cases.abs() < 1e-15 {
        return 0.0;
    }
    deaths / cases
}

pub fn viral_load_dynamics(v0: f64, growth_rate: f64, clearance_rate: f64, time: f64) -> f64 {
    v0 * ((growth_rate - clearance_rate) * time).exp()
}
