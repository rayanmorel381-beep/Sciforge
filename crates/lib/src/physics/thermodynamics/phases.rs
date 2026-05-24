use crate::constants::{K_B, R_GAS};

pub fn lever_rule(c0: f64, c_alpha: f64, c_beta: f64) -> (f64, f64) {
    let f_alpha = (c_beta - c0) / (c_beta - c_alpha);
    let f_beta = 1.0 - f_alpha;
    (f_alpha, f_beta)
}

pub fn gibbs_phase_rule(c: u32, p: u32) -> i32 {
    c as i32 - p as i32 + 2
}

pub fn clausius_clapeyron_slope(delta_h: f64, t: f64, delta_v: f64) -> f64 {
    delta_h / (t * delta_v)
}

pub fn regular_solution_gibbs(xa: f64, omega: f64, t: f64) -> f64 {
    let xb = 1.0 - xa;
    let mix_h = omega * xa * xb;
    let mix_s = -R_GAS * (xa * xa.max(1e-30).ln() + xb * xb.max(1e-30).ln());
    mix_h - t * mix_s
}

pub fn spinodal_temperature(omega: f64) -> f64 {
    omega / (2.0 * R_GAS)
}

pub fn nucleation_barrier(gamma: f64, delta_gv: f64) -> f64 {
    16.0 * std::f64::consts::PI * gamma.powi(3) / (3.0 * delta_gv * delta_gv)
}

pub fn critical_nucleus_radius(gamma: f64, delta_gv: f64) -> f64 {
    -2.0 * gamma / delta_gv
}

pub fn nucleation_rate(n0: f64, delta_g_star: f64, t: f64) -> f64 {
    n0 * (-delta_g_star / (K_B * t)).exp()
}

pub fn coarsening_rate(k: f64, t: f64, t0: f64) -> f64 {
    (k * (t - t0)).powf(1.0 / 3.0)
}

pub fn jmak(k: f64, t: f64, n: f64) -> f64 {
    1.0 - (-k * t.powf(n)).exp()
}

pub fn partition_coefficient(c_solid: f64, c_liquid: f64) -> f64 {
    c_solid / c_liquid.max(1e-30)
}

pub fn scheil_equation(c0: f64, k: f64, fs: f64) -> f64 {
    c0 * (1.0 - fs).powf(k - 1.0)
}
