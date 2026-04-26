pub fn lotka_volterra(
    prey: f64,
    pred: f64,
    alpha: f64,
    beta: f64,
    delta: f64,
    gamma: f64,
) -> (f64, f64) {
    let dprey = alpha * prey - beta * prey * pred;
    let dpred = delta * prey * pred - gamma * pred;
    (dprey, dpred)
}

pub fn lotka_volterra_solve(
    prey0: f64,
    pred0: f64,
    alpha: f64,
    beta: f64,
    delta: f64,
    gamma: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut prey = prey0;
    let mut pred = pred0;
    result.push((prey, pred));
    for _ in 0..steps {
        let (k1p, k1q) = lotka_volterra(prey, pred, alpha, beta, delta, gamma);
        let (k2p, k2q) = lotka_volterra(
            prey + 0.5 * dt * k1p,
            pred + 0.5 * dt * k1q,
            alpha,
            beta,
            delta,
            gamma,
        );
        let (k3p, k3q) = lotka_volterra(
            prey + 0.5 * dt * k2p,
            pred + 0.5 * dt * k2q,
            alpha,
            beta,
            delta,
            gamma,
        );
        let (k4p, k4q) =
            lotka_volterra(prey + dt * k3p, pred + dt * k3q, alpha, beta, delta, gamma);
        prey += dt / 6.0 * (k1p + 2.0 * k2p + 2.0 * k3p + k4p);
        pred += dt / 6.0 * (k1q + 2.0 * k2q + 2.0 * k3q + k4q);
        prey = prey.max(0.0);
        pred = pred.max(0.0);
        result.push((prey, pred));
    }
    result
}

pub fn competitive_lotka_volterra(
    n1: f64,
    n2: f64,
    r1: f64,
    r2: f64,
    k1: f64,
    k2: f64,
    a12: f64,
    a21: f64,
) -> (f64, f64) {
    let dn1 = r1 * n1 * (1.0 - (n1 + a12 * n2) / k1);
    let dn2 = r2 * n2 * (1.0 - (n2 + a21 * n1) / k2);
    (dn1, dn2)
}

pub fn holling_type_ii(prey: f64, attack_rate: f64, handling_time: f64) -> f64 {
    attack_rate * prey / (1.0 + attack_rate * handling_time * prey)
}

pub fn holling_type_iii(prey: f64, attack_rate: f64, handling_time: f64) -> f64 {
    attack_rate * prey * prey / (1.0 + attack_rate * handling_time * prey * prey)
}

pub fn rosenzweig_macarthur(
    prey: f64,
    pred: f64,
    r: f64,
    k: f64,
    a: f64,
    h: f64,
    e: f64,
    d: f64,
) -> (f64, f64) {
    let functional = a * prey / (1.0 + a * h * prey);
    let dprey = r * prey * (1.0 - prey / k) - functional * pred;
    let dpred = e * functional * pred - d * pred;
    (dprey, dpred)
}

pub fn beddington_deangelis(prey: f64, pred: f64, a: f64, b: f64, c: f64) -> f64 {
    a * prey / (1.0 + b * prey + c * pred)
}

pub fn ratio_dependent(prey: f64, pred: f64, a: f64, h: f64) -> f64 {
    if pred < 1e-30 {
        return 0.0;
    }
    let ratio = prey / pred;
    a * ratio / (1.0 + a * h * ratio)
}

pub fn intraguild_predation(
    prey: f64,
    meso: f64,
    top: f64,
    r: f64,
    k: f64,
    a_mp: f64,
    a_tp: f64,
    a_tm: f64,
    e_mp: f64,
    e_tp: f64,
    e_tm: f64,
    d_m: f64,
    d_t: f64,
) -> (f64, f64, f64) {
    let dp = r * prey * (1.0 - prey / k) - a_mp * meso * prey - a_tp * top * prey;
    let dm = e_mp * a_mp * meso * prey - a_tm * top * meso - d_m * meso;
    let dt = e_tp * a_tp * top * prey + e_tm * a_tm * top * meso - d_t * top;
    (dp, dm, dt)
}

pub fn apparent_competition(
    prey1: f64,
    prey2: f64,
    pred: f64,
    a1: f64,
    a2: f64,
    r1: f64,
    r2: f64,
    k1: f64,
    k2: f64,
    e: f64,
    d: f64,
) -> (f64, f64, f64) {
    let dp1 = r1 * prey1 * (1.0 - prey1 / k1) - a1 * pred * prey1;
    let dp2 = r2 * prey2 * (1.0 - prey2 / k2) - a2 * pred * prey2;
    let dpred = e * (a1 * prey1 + a2 * prey2) * pred - d * pred;
    (dp1, dp2, dpred)
}
