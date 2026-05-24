pub fn surface_tension_young(gamma_sv: f64, gamma_sl: f64, cos_theta: f64) -> f64 {
    (gamma_sv - gamma_sl) / cos_theta.abs().max(1e-30)
}

pub fn contact_angle(gamma_sv: f64, gamma_sl: f64, gamma_lv: f64) -> f64 {
    ((gamma_sv - gamma_sl) / gamma_lv.max(1e-30)).acos()
}

pub fn capillary_rise(gamma: f64, cos_theta: f64, rho: f64, g: f64, radius: f64) -> f64 {
    2.0 * gamma * cos_theta / (rho * g * radius).max(1e-30)
}

pub fn laplace_pressure(gamma: f64, r1: f64, r2: f64) -> f64 {
    gamma * (1.0 / r1.max(1e-30) + 1.0 / r2.max(1e-30))
}

pub fn gibbs_adsorption(d_gamma: f64, d_ln_concentration: f64, temperature: f64) -> f64 {
    -d_gamma / (crate::constants::R_GAS * temperature * d_ln_concentration).max(1e-30)
}

pub fn spreading_coefficient(gamma_sv: f64, gamma_lv: f64, gamma_sl: f64) -> f64 {
    gamma_sv - gamma_lv - gamma_sl
}

pub fn work_of_adhesion(gamma_lv: f64, cos_theta: f64) -> f64 {
    gamma_lv * (1.0 + cos_theta)
}
