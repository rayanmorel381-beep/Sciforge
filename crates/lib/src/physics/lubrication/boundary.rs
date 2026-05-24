pub fn stribeck_friction_coefficient(
    velocity_m_per_s: f64,
    eta_pa_s: f64,
    load_per_area_pa: f64,
    boundary_friction: f64,
    hydrodynamic_friction: f64,
    transition_lambda: f64,
) -> f64 {
    let lambda = eta_pa_s * velocity_m_per_s / load_per_area_pa;
    let ratio = lambda / transition_lambda;
    boundary_friction / (1.0 + ratio) + hydrodynamic_friction * ratio / (1.0 + ratio)
}

pub fn boundary_film_load_capacity(
    yield_stress_pa: f64,
    contact_area_m2: f64,
    asperity_fraction: f64,
) -> f64 {
    yield_stress_pa * contact_area_m2 * asperity_fraction
}

pub fn lambda_film_parameter(
    central_film_m: f64,
    rms_roughness_1_m: f64,
    rms_roughness_2_m: f64,
) -> f64 {
    central_film_m / (rms_roughness_1_m.powi(2) + rms_roughness_2_m.powi(2)).sqrt()
}
