use crate::constants::R_GAS;

pub fn ideal_gas_pressure(n: f64, t: f64, v: f64) -> f64 {
    n * R_GAS * t / v.max(1e-30)
}

pub fn ideal_gas_volume(n: f64, t: f64, p: f64) -> f64 {
    n * R_GAS * t / p.max(1e-30)
}

pub fn ideal_gas_temperature(p: f64, v: f64, n: f64) -> f64 {
    p * v / (n * R_GAS).max(1e-30)
}

pub fn boyles_law(p1: f64, v1: f64, v2: f64) -> f64 {
    p1 * v1 / v2.max(1e-30)
}

pub fn charles_law(v1: f64, t1: f64, t2: f64) -> f64 {
    v1 * t2 / t1.max(1e-30)
}

pub fn gay_lussac_law(p1: f64, t1: f64, t2: f64) -> f64 {
    p1 * t2 / t1.max(1e-30)
}

pub fn combined_gas_law(p1: f64, v1: f64, t1: f64, t2: f64, p2: f64) -> f64 {
    p1 * v1 * t2 / (t1 * p2).max(1e-30)
}

pub fn dalton_partial_pressure(mole_fraction: f64, total_pressure: f64) -> f64 {
    mole_fraction * total_pressure
}

pub fn grahams_law_effusion(m1: f64, m2: f64) -> f64 {
    (m2 / m1.max(1e-30)).sqrt()
}

pub fn gas_density(p: f64, mw: f64, t: f64) -> f64 {
    p * mw / (R_GAS * t).max(1e-30)
}

pub fn rms_speed(t: f64, mw: f64) -> f64 {
    (3.0 * R_GAS * t / mw.max(1e-30)).sqrt()
}

pub fn mean_speed(t: f64, mw: f64) -> f64 {
    (8.0 * R_GAS * t / (std::f64::consts::PI * mw).max(1e-30)).sqrt()
}

pub fn most_probable_speed(t: f64, mw: f64) -> f64 {
    (2.0 * R_GAS * t / mw.max(1e-30)).sqrt()
}

pub fn mean_free_path(d: f64, n_over_v: f64) -> f64 {
    1.0 / (std::f64::consts::SQRT_2 * std::f64::consts::PI * d * d * n_over_v).max(1e-30)
}
