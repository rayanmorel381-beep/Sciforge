use crate::constants::astronomy::EARTH_GRAVITY;

pub fn pendulum_natural_frequency_leg(leg_length_m: f64) -> f64 {
    (EARTH_GRAVITY / leg_length_m).sqrt() / (2.0 * std::f64::consts::PI)
}

pub fn ground_reaction_peak_force(
    body_mass_kg: f64,
    impact_velocity_m_per_s: f64,
    contact_time_s: f64,
) -> f64 {
    body_mass_kg * impact_velocity_m_per_s / contact_time_s
        + body_mass_kg * EARTH_GRAVITY
}

pub fn cost_of_transport(
    metabolic_power_w: f64,
    body_weight_n: f64,
    velocity_m_per_s: f64,
) -> f64 {
    metabolic_power_w / (body_weight_n * velocity_m_per_s.max(1e-12))
}

pub fn froude_number(velocity_m_per_s: f64, leg_length_m: f64) -> f64 {
    velocity_m_per_s * velocity_m_per_s / (EARTH_GRAVITY * leg_length_m)
}

pub fn stride_length_walking(leg_length_m: f64, froude: f64) -> f64 {
    2.0 * leg_length_m * froude.sqrt()
}
