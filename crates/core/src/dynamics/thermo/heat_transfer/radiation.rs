use sciforge_hub::prelude::constants::{SIGMA_SB, WIEN_DISPLACEMENT};

pub fn emissive_power_w_m2(emissivity: f64, temperature_k: f64) -> f64 {
    emissivity * SIGMA_SB * temperature_k.powi(4)
}

pub fn net_radiation_w(
    emissivity: f64,
    area_m2: f64,
    t_hot_k: f64,
    t_cold_k: f64,
) -> f64 {
    emissivity * SIGMA_SB * area_m2 * (t_hot_k.powi(4) - t_cold_k.powi(4))
}

pub fn radiation_resistance_k_w(
    emissivity: f64,
    area_m2: f64,
    t_hot_k: f64,
    t_cold_k: f64,
) -> f64 {
    let q = net_radiation_w(emissivity, area_m2, t_hot_k, t_cold_k);
    if q.abs() < 1e-12 {
        f64::INFINITY
    } else {
        (t_hot_k - t_cold_k) / q
    }
}

pub fn gray_body_exchange_w(
    area_m2: f64,
    eps1: f64,
    eps2: f64,
    view_factor: f64,
    t1_k: f64,
    t2_k: f64,
) -> f64 {
    let f = 1.0 / (1.0 / eps1 + 1.0 / eps2 - 1.0);
    SIGMA_SB * area_m2 * f * view_factor * (t1_k.powi(4) - t2_k.powi(4))
}

pub fn radiation_heat_transfer_coefficient_w_m2k(
    emissivity: f64,
    t_surface_k: f64,
    t_ambient_k: f64,
) -> f64 {
    emissivity
        * SIGMA_SB
        * (t_surface_k + t_ambient_k)
        * (t_surface_k * t_surface_k + t_ambient_k * t_ambient_k)
}

pub fn wien_peak_wavelength_m(temperature_k: f64) -> f64 {
    WIEN_DISPLACEMENT / temperature_k
}
