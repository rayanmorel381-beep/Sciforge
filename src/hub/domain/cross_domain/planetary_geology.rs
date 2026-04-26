//! Planetary geology: crater scaling laws, tidal dissipation,
//! regolith gardening, and surface age estimation.

use crate::constants::astronomy::TIDAL_DISSIPATION_COEFF;
use crate::constants::geology::{
    CRATER_DENSITY_EXPONENT, CRATER_GRAVITY_EXPONENT, CRATER_PROJECTILE_EXPONENT,
    CRATER_SCALING_COEFF, CRATER_VELOCITY_EXPONENT,
};
use crate::constants::physics::{G, SIGMA_SB};

pub fn impact_energy(projectile_mass: f64, impact_velocity: f64) -> f64 {
    0.5 * projectile_mass * impact_velocity * impact_velocity
}

pub fn crater_scaling_diameter(
    projectile_diameter: f64,
    projectile_density: f64,
    target_density: f64,
    velocity: f64,
    gravity: f64,
) -> f64 {
    CRATER_SCALING_COEFF
        * (projectile_density / target_density).powf(CRATER_DENSITY_EXPONENT)
        * projectile_diameter.powf(CRATER_PROJECTILE_EXPONENT)
        * velocity.powf(CRATER_VELOCITY_EXPONENT)
        * gravity.powf(CRATER_GRAVITY_EXPONENT)
}

pub fn tidal_heating_rate(
    eccentricity: f64,
    mean_motion: f64,
    planet_radius: f64,
    quality_param: f64,
    primary_mass: f64,
    semi_major: f64,
) -> f64 {
    TIDAL_DISSIPATION_COEFF
        * G
        * primary_mass
        * primary_mass
        * planet_radius.powi(5)
        * mean_motion
        * eccentricity
        * eccentricity
        / (quality_param * semi_major.powi(6))
}

pub fn surface_temperature_equilibrium(solar_flux: f64, albedo: f64, emissivity: f64) -> f64 {
    (solar_flux * (1.0 - albedo) / (4.0 * emissivity * SIGMA_SB)).powf(0.25)
}

pub fn lava_flow_cooling_time(thickness: f64, thermal_diffusivity: f64) -> f64 {
    thickness * thickness / (std::f64::consts::PI * std::f64::consts::PI * thermal_diffusivity)
}

pub fn regolith_depth(flux_rate: f64, surface_density: f64, time: f64) -> f64 {
    flux_rate * time / surface_density
}

pub fn lithospheric_thickness(
    thermal_conductivity: f64,
    heat_flow: f64,
    base_temp: f64,
    surface_temp: f64,
) -> f64 {
    thermal_conductivity * (base_temp - surface_temp) / heat_flow
}

pub fn gravitational_differentiation_time(
    radius: f64,
    density_diff: f64,
    gravity: f64,
    viscosity: f64,
) -> f64 {
    18.0 * viscosity / (density_diff * gravity * radius)
}

pub fn crater_counting_surface_age(crater_density: f64, production_rate: f64) -> f64 {
    crater_density / production_rate
}

pub fn volcanic_effusion_rate(
    thermal_flux: f64,
    specific_heat: f64,
    delta_t: f64,
    latent_heat: f64,
) -> f64 {
    thermal_flux / (specific_heat * delta_t + latent_heat)
}

pub fn ejecta_blanket_thickness(
    crater_radius: f64,
    distance_from_center: f64,
    rim_thickness: f64,
) -> f64 {
    rim_thickness * (distance_from_center / crater_radius).powf(-3.0)
}
