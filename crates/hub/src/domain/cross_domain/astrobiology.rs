//! Astrobiology: habitable zones, biosignature detection, and
//! planetary habitability models.

use sciforge_lib::constants::astronomy::{
    HABITABLE_ZONE_INNER_FLUX, HABITABLE_ZONE_OUTER_FLUX, SOLAR_LUMINOSITY,
};
use sciforge_lib::constants::physics::{G, K_B, SIGMA_SB};

pub fn planet_equilibrium_temperature(stellar_luminosity: f64, distance: f64, albedo: f64) -> f64 {
    (stellar_luminosity * (1.0 - albedo)
        / (16.0 * std::f64::consts::PI * distance * distance * SIGMA_SB))
        .powf(0.25)
}

pub fn habitable_zone_inner(luminosity: f64) -> f64 {
    (luminosity / (HABITABLE_ZONE_INNER_FLUX * SOLAR_LUMINOSITY)).sqrt()
}

pub fn habitable_zone_outer(luminosity: f64) -> f64 {
    (luminosity / (HABITABLE_ZONE_OUTER_FLUX * SOLAR_LUMINOSITY)).sqrt()
}

pub fn atmospheric_escape_parameter(
    temperature: f64,
    planet_mass: f64,
    planet_radius: f64,
    molecular_mass: f64,
) -> f64 {
    G * planet_mass * molecular_mass / (K_B * temperature * planet_radius)
}

pub fn tidal_locking_timescale(
    planet_mass: f64,
    semi_major: f64,
    star_mass: f64,
    planet_radius: f64,
    tidal_quality: f64,
) -> f64 {
    planet_mass * semi_major.powi(6) * tidal_quality
        / (G * star_mass * star_mass * planet_radius.powi(5))
}

pub fn energy_limited_mass_loss(
    xuv_flux: f64,
    efficiency: f64,
    planet_mass: f64,
    planet_radius: f64,
) -> f64 {
    efficiency * xuv_flux * std::f64::consts::PI * planet_radius.powi(3) / (G * planet_mass)
}

pub fn biosignature_column_density(
    mixing_ratio: f64,
    surface_pressure: f64,
    gravity: f64,
    mean_molecular_mass: f64,
) -> f64 {
    mixing_ratio * surface_pressure / (mean_molecular_mass * gravity)
}

pub fn uv_surface_flux(incident_flux: f64, optical_depth: f64) -> f64 {
    incident_flux * (-optical_depth).exp()
}

pub fn photosynthetic_flux_limit(
    photon_flux: f64,
    quantum_efficiency: f64,
    photon_energy: f64,
) -> f64 {
    photon_flux * quantum_efficiency * photon_energy
}

pub fn drake_equation(
    rate_star_formation: f64,
    fraction_planets: f64,
    habitable_per_system: f64,
    fraction_life: f64,
    fraction_intelligence: f64,
    fraction_communication: f64,
    civilization_lifetime: f64,
) -> f64 {
    rate_star_formation
        * fraction_planets
        * habitable_per_system
        * fraction_life
        * fraction_intelligence
        * fraction_communication
        * civilization_lifetime
}

pub fn surface_gravity(mass: f64, radius: f64) -> f64 {
    G * mass / (radius * radius)
}
