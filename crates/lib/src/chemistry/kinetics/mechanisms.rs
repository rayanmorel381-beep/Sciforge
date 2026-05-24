pub fn michaelis_menten(vmax: f64, km: f64, s: f64) -> f64 {
    vmax * s / (km + s)
}

pub fn lineweaver_burk_slope(km: f64, vmax: f64) -> f64 {
    km / vmax.max(1e-30)
}

pub fn lineweaver_burk_intercept(vmax: f64) -> f64 {
    1.0 / vmax.max(1e-30)
}

pub fn lindemann_unimolecular(k_inf: f64, k0: f64, m: f64) -> f64 {
    k0 * m / (1.0 + k0 * m / k_inf)
}

pub fn consecutive_reaction(c0: f64, k1: f64, k2: f64, t: f64) -> (f64, f64, f64) {
    let a = c0 * (-k1 * t).exp();
    let b = c0 * k1 / (k2 - k1) * ((-k1 * t).exp() - (-k2 * t).exp());
    let c = c0 - a - b;
    (a, b, c)
}

pub fn reversible_first_order(c0: f64, kf: f64, kr: f64, t: f64) -> (f64, f64) {
    let ceq = c0 * kf / (kf + kr);
    let a = c0 - ceq;
    let ca = ceq + a * (-(kf + kr) * t).exp();
    let cb = c0 - ca;
    (ca, cb)
}

pub fn parallel_reactions(c0: f64, k_values: &[f64], t: f64) -> Vec<f64> {
    let k_total: f64 = k_values.iter().sum();
    k_values
        .iter()
        .map(|&ki| c0 * ki / k_total.max(1e-30) * (1.0 - (-k_total * t).exp()))
        .collect()
}

pub fn steady_state_intermediate(k1: f64, k_minus1: f64, k2: f64, a: f64) -> f64 {
    k1 * a / (k_minus1 + k2)
}

pub fn pre_equilibrium_rate(k1: f64, k_minus1: f64, k2: f64, a: f64, b: f64) -> f64 {
    k1 * k2 / k_minus1.max(1e-30) * a * b
}

pub fn catalytic_efficiency(kcat: f64, km: f64) -> f64 {
    kcat / km.max(1e-30)
}

pub fn competitive_inhibition(vmax: f64, km: f64, s: f64, i: f64, ki: f64) -> f64 {
    vmax * s / (km * (1.0 + i / ki.max(1e-30)) + s)
}

pub fn uncompetitive_inhibition(vmax: f64, km: f64, s: f64, i: f64, ki: f64) -> f64 {
    let alpha_prime = 1.0 + i / ki.max(1e-30);
    vmax * s / (km + alpha_prime * s)
}
