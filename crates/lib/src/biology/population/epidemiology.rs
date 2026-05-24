pub fn sir_model(s: f64, i: f64, r: f64, beta: f64, gamma: f64) -> (f64, f64, f64) {
    let n = s + i + r;
    let ds = -beta * s * i / n;
    let di = beta * s * i / n - gamma * i;
    let dr = gamma * i;
    (ds, di, dr)
}

pub fn sir_solve(
    s0: f64,
    i0: f64,
    r0: f64,
    beta: f64,
    gamma: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut s = s0;
    let mut i = i0;
    let mut r = r0;
    result.push((s, i, r));
    for _ in 0..steps {
        let (k1s, k1i, k1r) = sir_model(s, i, r, beta, gamma);
        let (k2s, k2i, k2r) = sir_model(
            s + 0.5 * dt * k1s,
            i + 0.5 * dt * k1i,
            r + 0.5 * dt * k1r,
            beta,
            gamma,
        );
        let (k3s, k3i, k3r) = sir_model(
            s + 0.5 * dt * k2s,
            i + 0.5 * dt * k2i,
            r + 0.5 * dt * k2r,
            beta,
            gamma,
        );
        let (k4s, k4i, k4r) = sir_model(s + dt * k3s, i + dt * k3i, r + dt * k3r, beta, gamma);
        s += dt / 6.0 * (k1s + 2.0 * k2s + 2.0 * k3s + k4s);
        i += dt / 6.0 * (k1i + 2.0 * k2i + 2.0 * k3i + k4i);
        r += dt / 6.0 * (k1r + 2.0 * k2r + 2.0 * k3r + k4r);
        s = s.max(0.0);
        i = i.max(0.0);
        r = r.max(0.0);
        result.push((s, i, r));
    }
    result
}

pub fn seir_model(
    s: f64,
    e: f64,
    i: f64,
    r: f64,
    beta: f64,
    sigma: f64,
    gamma: f64,
) -> (f64, f64, f64, f64) {
    let n = s + e + i + r;
    let ds = -beta * s * i / n;
    let de = beta * s * i / n - sigma * e;
    let di = sigma * e - gamma * i;
    let dr = gamma * i;
    (ds, de, di, dr)
}

pub fn seir_solve(
    s0: f64,
    e0: f64,
    i0: f64,
    r0: f64,
    beta: f64,
    sigma: f64,
    gamma: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64, f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut s, mut e, mut i, mut r) = (s0, e0, i0, r0);
    result.push((s, e, i, r));
    for _ in 0..steps {
        let (ds, de, di, dr) = seir_model(s, e, i, r, beta, sigma, gamma);
        s += ds * dt;
        e += de * dt;
        i += di * dt;
        r += dr * dt;
        s = s.max(0.0);
        e = e.max(0.0);
        i = i.max(0.0);
        r = r.max(0.0);
        result.push((s, e, i, r));
    }
    result
}

pub fn sis_model(s: f64, i: f64, beta: f64, gamma: f64) -> (f64, f64) {
    let n = s + i;
    let ds = -beta * s * i / n + gamma * i;
    let di = beta * s * i / n - gamma * i;
    (ds, di)
}

pub fn sirs_model(s: f64, i: f64, r: f64, beta: f64, gamma: f64, xi: f64) -> (f64, f64, f64) {
    let n = s + i + r;
    let ds = -beta * s * i / n + xi * r;
    let di = beta * s * i / n - gamma * i;
    let dr = gamma * i - xi * r;
    (ds, di, dr)
}

pub fn basic_reproduction_number(beta: f64, gamma: f64) -> f64 {
    beta / gamma
}

pub fn herd_immunity_threshold(r0: f64) -> f64 {
    1.0 - 1.0 / r0
}

pub fn final_size_equation(r0: f64, tolerance: f64, max_iter: usize) -> f64 {
    let mut r_inf = 0.99;
    for _ in 0..max_iter {
        let new_r = 1.0 - (-r0 * r_inf).exp();
        if (new_r - r_inf).abs() < tolerance {
            return new_r;
        }
        r_inf = new_r;
    }
    r_inf
}

pub fn generation_time(incubation: f64, infectious_period: f64) -> f64 {
    incubation + infectious_period / 2.0
}
