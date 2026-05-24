use crate::constants::{C, E_CHARGE, EPSILON_0, MU_0};
use std::f64::consts::PI;

pub fn lorentz_factor(velocity_m_per_s: f64) -> f64 {
    let beta_sq = (velocity_m_per_s / C).powi(2);
    1.0 / (1.0 - beta_sq).sqrt()
}

pub fn beta_factor(velocity_m_per_s: f64) -> f64 {
    velocity_m_per_s / C
}

pub fn e_field_transverse_boost(
    e_perp_v_per_m: f64,
    b_perp_t: f64,
    velocity_m_per_s: f64,
    boost_direction_sign: f64,
) -> f64 {
    let gamma = lorentz_factor(velocity_m_per_s);
    gamma * (e_perp_v_per_m + boost_direction_sign * velocity_m_per_s * b_perp_t)
}

pub fn b_field_transverse_boost(
    b_perp_t: f64,
    e_perp_v_per_m: f64,
    velocity_m_per_s: f64,
    boost_direction_sign: f64,
) -> f64 {
    let gamma = lorentz_factor(velocity_m_per_s);
    gamma * (b_perp_t - boost_direction_sign * velocity_m_per_s * e_perp_v_per_m / (C * C))
}

pub fn doppler_shift_relativistic_em(
    source_frequency_hz: f64,
    velocity_m_per_s: f64,
    angle_rad: f64,
) -> f64 {
    let beta = velocity_m_per_s / C;
    let gamma = 1.0 / (1.0 - beta * beta).sqrt();
    source_frequency_hz / (gamma * (1.0 - beta * angle_rad.cos()))
}

pub fn aberration_angle(angle_source_rad: f64, velocity_m_per_s: f64) -> f64 {
    let beta = velocity_m_per_s / C;
    let cos_t = (angle_source_rad.cos() - beta) / (1.0 - beta * angle_source_rad.cos());
    cos_t.acos()
}

pub fn lienard_wiechert_potential_scalar(
    charge_c: f64,
    distance_m: f64,
    velocity_m_per_s: f64,
    angle_rad: f64,
) -> f64 {
    let beta = velocity_m_per_s / C;
    charge_c / (4.0 * PI * EPSILON_0 * distance_m * (1.0 - beta * angle_rad.cos()))
}

pub fn lienard_wiechert_vector_magnitude(
    charge_c: f64,
    velocity_m_per_s: f64,
    distance_m: f64,
    angle_rad: f64,
) -> f64 {
    let beta = velocity_m_per_s / C;
    MU_0 * charge_c * velocity_m_per_s
        / (4.0 * PI * distance_m * (1.0 - beta * angle_rad.cos()))
}

pub fn larmor_power_relativistic(
    acceleration_m_per_s2_parallel: f64,
    acceleration_m_per_s2_perp: f64,
    velocity_m_per_s: f64,
) -> f64 {
    let gamma = lorentz_factor(velocity_m_per_s);
    let prefactor = E_CHARGE * E_CHARGE / (6.0 * PI * EPSILON_0 * C.powi(3));
    prefactor * gamma.powi(6)
        * (acceleration_m_per_s2_parallel.powi(2)
            + acceleration_m_per_s2_perp.powi(2) / (gamma * gamma))
}

pub fn synchrotron_critical_frequency(
    gamma_lorentz: f64,
    radius_m: f64,
) -> f64 {
    3.0 * gamma_lorentz.powi(3) * C / (4.0 * PI * radius_m)
}

pub fn synchrotron_total_power(
    gamma_lorentz: f64,
    radius_m: f64,
) -> f64 {
    let prefactor = E_CHARGE * E_CHARGE * C / (6.0 * PI * EPSILON_0);
    prefactor * gamma_lorentz.powi(4) / (radius_m * radius_m)
}

pub fn synchrotron_energy_loss_per_turn_ev(
    gamma_lorentz: f64,
    radius_m: f64,
) -> f64 {
    let prefactor = E_CHARGE / (3.0 * EPSILON_0);
    prefactor * gamma_lorentz.powi(4) / radius_m / E_CHARGE
}

pub fn cherenkov_angle(refractive_index: f64, velocity_m_per_s: f64) -> f64 {
    let beta = velocity_m_per_s / C;
    (1.0 / (refractive_index * beta)).acos()
}

pub fn cherenkov_threshold_velocity(refractive_index: f64) -> f64 {
    C / refractive_index
}

pub fn cherenkov_photons_per_unit_length(
    charge_number: f64,
    refractive_index: f64,
    velocity_m_per_s: f64,
    wavelength_min_m: f64,
    wavelength_max_m: f64,
) -> f64 {
    let alpha = crate::constants::ALPHA_FINE;
    let beta = velocity_m_per_s / C;
    let sin2 = 1.0 - 1.0 / (refractive_index * beta).powi(2);
    if sin2 <= 0.0 {
        0.0
    } else {
        2.0 * PI * alpha * charge_number * charge_number * sin2
            * (1.0 / wavelength_min_m - 1.0 / wavelength_max_m)
    }
}

pub fn relativistic_cyclotron_frequency(
    charge_c: f64,
    b_t: f64,
    mass_kg: f64,
    velocity_m_per_s: f64,
) -> f64 {
    let gamma = lorentz_factor(velocity_m_per_s);
    charge_c.abs() * b_t / (gamma * mass_kg)
}

pub fn thomas_precession_frequency(
    acceleration_m_per_s2: f64,
    velocity_m_per_s: f64,
) -> f64 {
    let gamma = lorentz_factor(velocity_m_per_s);
    (gamma * gamma / (gamma + 1.0)) * (acceleration_m_per_s2 * velocity_m_per_s) / (C * C)
}

pub fn invariant_em_field(
    e_v_per_m: f64,
    b_t: f64,
) -> f64 {
    e_v_per_m * e_v_per_m - C * C * b_t * b_t
}

pub fn invariant_e_dot_b(
    ex: f64, ey: f64, ez: f64,
    bx: f64, by: f64, bz: f64,
) -> f64 {
    ex * bx + ey * by + ez * bz
}
