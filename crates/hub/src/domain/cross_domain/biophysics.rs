//! Biophysics: Stokes-Einstein diffusion, Debye-Hückel theory,
//! membrane potentials, and macromolecular electrostatics.

use sciforge_lib::constants::physics::{E_CHARGE, EPSILON_0, K_B, N_A};

pub fn diffusion_coefficient_stokes_einstein(temperature: f64, viscosity: f64, radius: f64) -> f64 {
    K_B * temperature / (6.0 * std::f64::consts::PI * viscosity * radius)
}

pub fn membrane_capacitance(area: f64, thickness: f64, dielectric_constant: f64) -> f64 {
    EPSILON_0 * dielectric_constant * area / thickness
}

pub fn stokes_drag_force(viscosity: f64, radius: f64, velocity: f64) -> f64 {
    6.0 * std::f64::consts::PI * viscosity * radius * velocity
}

pub fn sedimentation_coefficient(
    particle_mass: f64,
    solvent_density: f64,
    particle_density: f64,
    friction_coefficient: f64,
) -> f64 {
    particle_mass * (1.0 - solvent_density / particle_density) / friction_coefficient
}

pub fn thermal_fluctuation_amplitude(temperature: f64, spring_constant: f64) -> f64 {
    (K_B * temperature / spring_constant).sqrt()
}

pub fn worm_like_chain_extension(
    force: f64,
    contour_length: f64,
    persistence_length: f64,
    temperature: f64,
) -> f64 {
    let x = force * persistence_length / (K_B * temperature);
    contour_length * (1.0 - 0.5 / x.sqrt() + x) / (1.0 + x)
}

pub fn reynolds_number(density: f64, velocity: f64, length: f64, viscosity: f64) -> f64 {
    density * velocity * length / viscosity
}

pub fn fick_diffusion_flux(diffusion_coeff: f64, concentration_gradient: f64) -> f64 {
    -diffusion_coeff * concentration_gradient
}

pub fn debye_screening_length(
    temperature: f64,
    ionic_strength: f64,
    dielectric_constant: f64,
) -> f64 {
    (EPSILON_0 * dielectric_constant * K_B * temperature
        / (2.0 * N_A * E_CHARGE * E_CHARGE * ionic_strength))
        .sqrt()
}

pub fn electrophoretic_mobility(charge: f64, friction_coefficient: f64) -> f64 {
    charge / friction_coefficient
}

pub fn helfrich_bending_energy(
    bending_modulus: f64,
    mean_curvature: f64,
    spontaneous_curvature: f64,
    area: f64,
) -> f64 {
    0.5 * bending_modulus * (2.0 * mean_curvature - spontaneous_curvature).powi(2) * area
}
