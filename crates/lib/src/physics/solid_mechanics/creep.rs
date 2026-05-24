use crate::constants::R_GAS;

pub fn norton_strain_rate(a: f64, sigma_pa: f64, n: f64, q_j_per_mol: f64, t_k: f64) -> f64 {
    a * sigma_pa.powf(n) * (-q_j_per_mol / (R_GAS * t_k)).exp()
}

pub fn larson_miller_parameter(t_k: f64, time_h: f64, c: f64) -> f64 {
    t_k * (c + time_h.log10())
}

pub fn time_to_rupture_arrhenius(a: f64, sigma_pa: f64, n: f64, q_j_per_mol: f64, t_k: f64) -> f64 {
    1.0 / (a * sigma_pa.powf(n) * (-q_j_per_mol / (R_GAS * t_k)).exp())
}

pub fn monkman_grant(strain_rate_per_s: f64, m: f64, c_mg: f64) -> f64 {
    c_mg / strain_rate_per_s.powf(m)
}

pub fn time_from_larson_miller(lmp: f64, t_k: f64, c: f64) -> f64 {
    10f64.powf(lmp / t_k - c)
}
