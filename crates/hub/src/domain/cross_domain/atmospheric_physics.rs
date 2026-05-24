//! Atmospheric physics: black-body radiation, Wien displacement,
//! radiative transfer, and atmospheric thermodynamics.

use sciforge_lib::constants::astronomy::WIEN_DISPLACEMENT;
use sciforge_lib::constants::physics::{C, H, K_B, SIGMA_SB};

pub fn planck_radiance(wavelength: f64, temperature: f64) -> f64 {
    let a = 2.0 * H * C * C / wavelength.powi(5);
    let b = H * C / (wavelength * K_B * temperature);
    a / (b.exp() - 1.0)
}

pub fn stefan_boltzmann_flux(temperature: f64) -> f64 {
    SIGMA_SB * temperature.powi(4)
}

pub fn brightness_temperature(radiance: f64, wavelength: f64) -> f64 {
    let a = 2.0 * H * C * C / (wavelength.powi(5) * radiance);
    H * C / (wavelength * K_B * (a + 1.0).ln())
}

pub fn rayleigh_scattering_cross_section(
    wavelength: f64,
    refractive_index: f64,
    number_density: f64,
) -> f64 {
    let n2m1 = refractive_index * refractive_index - 1.0;
    8.0 * std::f64::consts::PI.powi(3) * n2m1 * n2m1
        / (3.0 * number_density * number_density * wavelength.powi(4))
}

pub fn optical_depth(cross_section: f64, number_density: f64, path_length: f64) -> f64 {
    cross_section * number_density * path_length
}

pub fn atmospheric_scale_height(temperature: f64, mean_molecular_mass: f64, gravity: f64) -> f64 {
    K_B * temperature / (mean_molecular_mass * gravity)
}

pub fn pressure_at_altitude(surface_pressure: f64, scale_height: f64, altitude: f64) -> f64 {
    surface_pressure * (-altitude / scale_height).exp()
}

pub fn dry_adiabatic_lapse_rate(gravity: f64, specific_heat: f64) -> f64 {
    gravity / specific_heat
}

pub fn wien_peak_wavelength(temperature: f64) -> f64 {
    WIEN_DISPLACEMENT / temperature
}

pub fn schwarzschild_radiative_transfer(
    source_function: f64,
    initial_radiance: f64,
    optical_depth: f64,
) -> f64 {
    initial_radiance * (-optical_depth).exp() + source_function * (1.0 - (-optical_depth).exp())
}

pub fn effective_emission_temperature(outgoing_flux: f64, emissivity: f64) -> f64 {
    (outgoing_flux / (emissivity * SIGMA_SB)).powf(0.25)
}
