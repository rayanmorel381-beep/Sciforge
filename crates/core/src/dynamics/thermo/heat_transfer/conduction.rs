use std::f64::consts::PI;

pub fn flat_wall_flux_w_m2(conductivity_w_mk: f64, thickness_m: f64, delta_t_k: f64) -> f64 {
    conductivity_w_mk * delta_t_k / thickness_m
}

pub fn flat_wall_resistance_k_w(
    conductivity_w_mk: f64,
    thickness_m: f64,
    area_m2: f64,
) -> f64 {
    thickness_m / (conductivity_w_mk * area_m2)
}

pub fn cylindrical_wall_flux_w_m(
    conductivity_w_mk: f64,
    r_inner_m: f64,
    r_outer_m: f64,
    delta_t_k: f64,
) -> f64 {
    2.0 * PI * conductivity_w_mk * delta_t_k / (r_outer_m / r_inner_m).ln()
}

pub fn cylindrical_wall_resistance_k_w(
    conductivity_w_mk: f64,
    r_inner_m: f64,
    r_outer_m: f64,
    length_m: f64,
) -> f64 {
    (r_outer_m / r_inner_m).ln() / (2.0 * PI * conductivity_w_mk * length_m)
}

pub fn heat_flow_w(thermal_resistance_k_w: f64, delta_t_k: f64) -> f64 {
    delta_t_k / thermal_resistance_k_w
}

pub fn series_resistance_k_w(resistances: &[f64]) -> f64 {
    resistances.iter().sum()
}

pub fn parallel_resistance_k_w(resistances: &[f64]) -> f64 {
    1.0 / resistances.iter().map(|r| 1.0 / r).sum::<f64>()
}

pub fn overall_heat_transfer_coefficient_w_m2k(resistances_per_m2: &[f64]) -> f64 {
    1.0 / resistances_per_m2.iter().sum::<f64>()
}
