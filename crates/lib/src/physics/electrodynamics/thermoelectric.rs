pub fn seebeck_voltage(seebeck_v_per_k: f64, delta_t_k: f64) -> f64 {
    seebeck_v_per_k * delta_t_k
}

pub fn peltier_coefficient(seebeck_v_per_k: f64, t_k: f64) -> f64 {
    seebeck_v_per_k * t_k
}

pub fn peltier_heat_rate(peltier_coeff_v: f64, current_a: f64) -> f64 {
    peltier_coeff_v * current_a
}

pub fn thomson_coefficient(dseebeck_dt_v_per_k2: f64, t_k: f64) -> f64 {
    t_k * dseebeck_dt_v_per_k2
}

pub fn thomson_heat_rate(
    thomson_coeff_v_per_k: f64,
    current_a: f64,
    grad_t_k_per_m: f64,
    length_m: f64,
) -> f64 {
    thomson_coeff_v_per_k * current_a * grad_t_k_per_m * length_m
}

pub fn figure_of_merit_zt(
    seebeck_v_per_k: f64,
    electrical_conductivity_s_per_m: f64,
    thermal_conductivity_w_per_m_per_k: f64,
    t_k: f64,
) -> f64 {
    seebeck_v_per_k * seebeck_v_per_k * electrical_conductivity_s_per_m * t_k
        / thermal_conductivity_w_per_m_per_k
}

pub fn power_factor(seebeck_v_per_k: f64, electrical_conductivity_s_per_m: f64) -> f64 {
    seebeck_v_per_k * seebeck_v_per_k * electrical_conductivity_s_per_m
}

pub fn carnot_efficiency(t_hot_k: f64, t_cold_k: f64) -> f64 {
    1.0 - t_cold_k / t_hot_k
}

pub fn thermoelectric_efficiency_max(
    t_hot_k: f64,
    t_cold_k: f64,
    zt_average: f64,
) -> f64 {
    let carnot = 1.0 - t_cold_k / t_hot_k;
    let m = (1.0 + zt_average).sqrt();
    carnot * (m - 1.0) / (m + t_cold_k / t_hot_k)
}

pub fn cop_thermoelectric_cooler(
    t_hot_k: f64,
    t_cold_k: f64,
    zt_average: f64,
) -> f64 {
    let m = (1.0 + zt_average).sqrt();
    let denom = m + 1.0;
    let num = m - t_hot_k / t_cold_k;
    (t_cold_k / (t_hot_k - t_cold_k)) * num / denom
}

pub fn onsager_heat_flux(
    l11: f64,
    l12: f64,
    grad_mu_per_m: f64,
    grad_t_k_per_m: f64,
) -> f64 {
    -(l11 * grad_mu_per_m + l12 * grad_t_k_per_m)
}

pub fn onsager_electric_current(
    l21: f64,
    l22: f64,
    grad_mu_per_m: f64,
    grad_t_k_per_m: f64,
) -> f64 {
    -(l21 * grad_mu_per_m + l22 * grad_t_k_per_m)
}

pub fn kelvin_relations_check(seebeck_v_per_k: f64, peltier_coeff_v: f64, t_k: f64) -> f64 {
    peltier_coeff_v - seebeck_v_per_k * t_k
}

pub fn nernst_voltage(nernst_coeff: f64, b_field_t: f64, grad_t_k_per_m: f64, length_m: f64) -> f64 {
    nernst_coeff * b_field_t * grad_t_k_per_m * length_m
}
