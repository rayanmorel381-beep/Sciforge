//! Geophysics: gravity anomalies, geothermal flux, magnetic field
//! models, and isostatic equilibrium.

use crate::constants::astronomy::{EARTH_GRAVITY, EARTH_RADIUS};
use crate::constants::geology::{HEAT_PRODUCTION_K40, HEAT_PRODUCTION_TH232, HEAT_PRODUCTION_U238};
use crate::constants::physics::{G, MU_0, SIGMA_SB};

pub fn radiogenic_heat_production(c_u238: f64, c_th232: f64, c_k40: f64, density: f64) -> f64 {
    density
        * (c_u238 * HEAT_PRODUCTION_U238
            + c_th232 * HEAT_PRODUCTION_TH232
            + c_k40 * HEAT_PRODUCTION_K40)
}

pub fn bouguer_anomaly(
    observed_gravity: f64,
    reference_gravity: f64,
    elevation: f64,
    slab_density: f64,
) -> f64 {
    let free_air = 2.0 * EARTH_GRAVITY * elevation / EARTH_RADIUS;
    let bouguer_correction = 2.0 * std::f64::consts::PI * G * slab_density * elevation;
    (observed_gravity - reference_gravity) + free_air - bouguer_correction
}

pub fn gravity_anomaly_buried_sphere(
    delta_rho: f64,
    radius: f64,
    center_depth: f64,
    horizontal_distance: f64,
) -> f64 {
    let volume_factor = 4.0 / 3.0 * std::f64::consts::PI * G * delta_rho * radius.powi(3);
    let r_sq = horizontal_distance.powi(2) + center_depth.powi(2);
    volume_factor * center_depth / r_sq.powf(1.5)
}

pub fn magnetic_anomaly_vertical_dipole(moment: f64, depth: f64, horizontal_distance: f64) -> f64 {
    let x2 = horizontal_distance.powi(2);
    let z2 = depth.powi(2);
    let r2 = x2 + z2;
    MU_0 * moment / (4.0 * std::f64::consts::PI) * (2.0 * z2 - x2) / r2.powf(2.5)
}

pub fn seismic_wave_attenuation(
    amplitude_0: f64,
    frequency: f64,
    travel_time: f64,
    quality_factor: f64,
) -> f64 {
    amplitude_0 * (-std::f64::consts::PI * frequency * travel_time / quality_factor).exp()
}

pub fn seismic_impedance_reflection(rho1: f64, v1: f64, rho2: f64, v2: f64) -> f64 {
    let z1 = rho1 * v1;
    let z2 = rho2 * v2;
    (z2 - z1) / (z2 + z1)
}

pub fn lithospheric_flexure_wavelength(
    flexural_rigidity: f64,
    mantle_density: f64,
    infill_density: f64,
    g_surface: f64,
) -> f64 {
    (4.0 * flexural_rigidity / ((mantle_density - infill_density) * g_surface)).powf(0.25)
}

pub fn isostatic_rebound_timescale(
    viscosity: f64,
    mantle_density: f64,
    g_surface: f64,
    wavelength: f64,
) -> f64 {
    4.0 * std::f64::consts::PI * viscosity / (mantle_density * g_surface * wavelength)
}

pub fn curie_depth(
    surface_heat_flow: f64,
    thermal_conductivity: f64,
    curie_temperature: f64,
    surface_temperature: f64,
) -> f64 {
    thermal_conductivity * (curie_temperature - surface_temperature) / surface_heat_flow
}

pub fn radiative_thermal_conductivity(
    temperature: f64,
    refractive_index: f64,
    absorption_coefficient: f64,
) -> f64 {
    16.0 * refractive_index.powi(2) * SIGMA_SB * temperature.powi(3)
        / (3.0 * absorption_coefficient)
}

pub fn electromagnetic_skin_depth(resistivity: f64, frequency: f64) -> f64 {
    (resistivity / (std::f64::consts::PI * MU_0 * frequency)).sqrt()
}
