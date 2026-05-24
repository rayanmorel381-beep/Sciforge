pub fn paschen_voltage(
    a_coefficient: f64,
    b_coefficient: f64,
    pressure_pa: f64,
    distance_m: f64,
    gamma_se: f64,
) -> f64 {
    let pd = pressure_pa * distance_m;
    let term = (1.0 + 1.0 / gamma_se).ln();
    b_coefficient * pd / ((a_coefficient * pd / term).ln())
}

pub fn paschen_minimum_pd(
    a_coefficient: f64,
    gamma_se: f64,
) -> f64 {
    let term = (1.0 + 1.0 / gamma_se).ln();
    std::f64::consts::E * term / a_coefficient
}

pub fn paschen_minimum_voltage(
    b_coefficient: f64,
    a_coefficient: f64,
    gamma_se: f64,
) -> f64 {
    let term = (1.0 + 1.0 / gamma_se).ln();
    b_coefficient * std::f64::consts::E * term / a_coefficient
}

pub fn townsend_first_coefficient(
    a_coefficient: f64,
    b_coefficient: f64,
    pressure_pa: f64,
    e_field_v_per_m: f64,
) -> f64 {
    a_coefficient * pressure_pa * (-b_coefficient * pressure_pa / e_field_v_per_m).exp()
}

pub fn townsend_breakdown_condition(
    alpha_per_m: f64,
    distance_m: f64,
    gamma_se: f64,
) -> f64 {
    gamma_se * ((alpha_per_m * distance_m).exp() - 1.0)
}

pub fn townsend_breakdown_distance(alpha_per_m: f64, gamma_se: f64) -> f64 {
    (1.0 + 1.0 / gamma_se).ln() / alpha_per_m
}

pub fn dielectric_breakdown_voltage(strength_v_per_m: f64, thickness_m: f64) -> f64 {
    strength_v_per_m * thickness_m
}

pub fn dielectric_breakdown_thickness(strength_v_per_m: f64, voltage_v: f64) -> f64 {
    voltage_v / strength_v_per_m
}

pub fn streamer_criterion_meek(
    alpha_per_m: f64,
    distance_m: f64,
    threshold: f64,
) -> bool {
    alpha_per_m * distance_m >= threshold
}

pub fn corona_onset_field_peek(
    radius_m: f64,
    relative_density: f64,
    surface_factor: f64,
) -> f64 {
    let e0 = 3.04e6;
    e0 * relative_density * surface_factor
        * (1.0 + 0.0301 / (relative_density * radius_m).sqrt())
}

pub fn ionization_rate_density(
    electron_density_per_m3: f64,
    drift_velocity_m_per_s: f64,
    alpha_per_m: f64,
) -> f64 {
    electron_density_per_m3 * drift_velocity_m_per_s * alpha_per_m
}

pub fn breakdown_field_pd_scaling(
    e_ref_v_per_m: f64,
    pd_ref: f64,
    pd: f64,
) -> f64 {
    e_ref_v_per_m * (pd_ref / pd).sqrt()
}
