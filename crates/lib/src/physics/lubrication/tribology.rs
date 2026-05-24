pub fn coulomb_friction_force(normal_force_n: f64, friction_coefficient: f64) -> f64 {
    friction_coefficient * normal_force_n
}

pub fn archard_wear_volume(
    normal_load_n: f64,
    sliding_distance_m: f64,
    hardness_pa: f64,
    wear_coefficient: f64,
) -> f64 {
    wear_coefficient * normal_load_n * sliding_distance_m / hardness_pa
}

pub fn frictional_heat_flux(
    friction_coefficient: f64,
    normal_pressure_pa: f64,
    sliding_velocity_m_per_s: f64,
) -> f64 {
    friction_coefficient * normal_pressure_pa * sliding_velocity_m_per_s
}

pub fn flash_temperature_blok(
    friction_coefficient: f64,
    normal_load_n: f64,
    velocity_m_per_s: f64,
    contact_radius_m: f64,
    thermal_conductivity_w_per_m_k: f64,
    density_kg_per_m3: f64,
    specific_heat_j_per_kg_k: f64,
) -> f64 {
    let q = friction_coefficient * normal_load_n * velocity_m_per_s
        / (std::f64::consts::PI * contact_radius_m * contact_radius_m);
    let peclet = velocity_m_per_s * contact_radius_m * density_kg_per_m3 * specific_heat_j_per_kg_k
        / thermal_conductivity_w_per_m_k;
    1.064 * q * contact_radius_m / (thermal_conductivity_w_per_m_k * peclet.sqrt())
}
