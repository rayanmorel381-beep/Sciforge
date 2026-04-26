use crate::constants::{EARTH_GRAVITY, K_B};

pub fn stokes_sedimentation(r: f64, rho_p: f64, rho_f: f64, viscosity: f64) -> f64 {
    2.0 * r * r * (rho_p - rho_f) * EARTH_GRAVITY / (9.0 * viscosity).max(1e-30)
}

pub fn brownian_diffusion_coefficient(t: f64, viscosity: f64, r: f64) -> f64 {
    K_B * t / (6.0 * std::f64::consts::PI * viscosity * r).max(1e-30)
}

pub fn einstein_diffusion_displacement(d: f64, t: f64) -> f64 {
    (2.0 * d * t).sqrt()
}

pub fn peclet_number_colloid(velocity: f64, r: f64, d: f64) -> f64 {
    velocity * r / d.max(1e-30)
}

pub fn osmotic_pressure_colloid(n_particles: f64, volume: f64, t: f64) -> f64 {
    n_particles * K_B * t / volume.max(1e-30)
}

pub fn sedimentation_coefficient(velocity: f64, omega: f64, r_centrifuge: f64) -> f64 {
    velocity / (omega * omega * r_centrifuge).max(1e-30)
}

pub fn tyndall_scattering_intensity(n: f64, v_particle: f64, wavelength: f64) -> f64 {
    n * v_particle * v_particle / wavelength.powi(4).max(1e-30)
}

pub fn specific_surface_area(radius: f64, density: f64) -> f64 {
    3.0 / (radius * density).max(1e-30)
}

pub fn flocculation_rate_smoluchowski(n0: f64, k_b: f64, t: f64, viscosity: f64) -> f64 {
    4.0 * k_b * t * n0 * n0 / (3.0 * viscosity).max(1e-30)
}

pub fn half_life_coagulation(n0: f64, k_rate: f64) -> f64 {
    1.0 / (k_rate * n0).max(1e-30)
}
