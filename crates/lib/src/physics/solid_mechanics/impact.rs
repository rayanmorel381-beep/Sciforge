use std::f64::consts::PI;

pub fn impact_velocity_drop(height_m: f64, g: f64) -> f64 {
    (2.0 * g * height_m).sqrt()
}

pub fn restitution_velocity(v_in: f64, e: f64) -> f64 {
    -e * v_in
}

pub fn hertz_impact_max_force(
    mass_kg: f64,
    velocity_m_per_s: f64,
    r_eq_m: f64,
    e_star_pa: f64,
) -> f64 {
    let k = (4.0 / 3.0) * e_star_pa * r_eq_m.sqrt();
    let f_max_pow = mass_kg * velocity_m_per_s.powi(2) * k.powf(2.0 / 5.0);
    1.139 * (f_max_pow).powf(3.0 / 5.0)
}

pub fn hertz_impact_duration(
    mass_kg: f64,
    velocity_m_per_s: f64,
    r_eq_m: f64,
    e_star_pa: f64,
) -> f64 {
    let k = (4.0 / 3.0) * e_star_pa * r_eq_m.sqrt();
    2.94 * (5.0 * mass_kg / (4.0 * k)).powf(2.0 / 5.0) * velocity_m_per_s.powf(-1.0 / 5.0)
}

pub fn charpy_energy_to_toughness(charpy_j: f64, area_m2: f64) -> f64 {
    charpy_j / area_m2
}

pub fn elastic_collision_velocities(
    m1: f64,
    v1: f64,
    m2: f64,
    v2: f64,
) -> (f64, f64) {
    let v1_p = ((m1 - m2) * v1 + 2.0 * m2 * v2) / (m1 + m2);
    let v2_p = ((m2 - m1) * v2 + 2.0 * m1 * v1) / (m1 + m2);
    (v1_p, v2_p)
}

pub fn inelastic_collision_velocity(m1: f64, v1: f64, m2: f64, v2: f64) -> f64 {
    (m1 * v1 + m2 * v2) / (m1 + m2)
}

pub fn restitution_kinetic_energy_lost(m1: f64, m2: f64, v_rel: f64, e: f64) -> f64 {
    0.5 * (m1 * m2) / (m1 + m2) * (1.0 - e * e) * v_rel * v_rel
}

pub fn split_hopkinson_strain_rate(c0: f64, gauge_strain: f64, length_m: f64) -> f64 {
    -2.0 * c0 * gauge_strain / length_m
}

pub fn pressure_wave_amplitude(rho: f64, c: f64, particle_velocity: f64) -> f64 {
    rho * c * particle_velocity
}

pub fn natural_frequency_lumped(k_n_per_m: f64, mass_kg: f64) -> f64 {
    (1.0 / (2.0 * PI)) * (k_n_per_m / mass_kg).sqrt()
}
