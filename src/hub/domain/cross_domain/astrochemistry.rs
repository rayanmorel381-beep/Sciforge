//! Astrochemistry: Jeans mass, molecular-cloud thermodynamics, and
//! interstellar chemical reaction rates.

use crate::constants::physics::{ELECTRON_MASS_KG, G, H, K_B, PROTON_MASS_KG, SIGMA_SB};

pub fn jeans_mass(temperature: f64, number_density: f64, mean_molecular_weight: f64) -> f64 {
    let mu_mh = mean_molecular_weight * PROTON_MASS_KG;
    let term1 = (5.0 * K_B * temperature / (G * mu_mh)).powf(1.5);
    let term2 = (3.0 / (4.0 * std::f64::consts::PI * number_density * mu_mh)).sqrt();
    term1 * term2
}

pub fn jeans_length(temperature: f64, number_density: f64, mean_molecular_weight: f64) -> f64 {
    let mu_mh = mean_molecular_weight * PROTON_MASS_KG;
    (std::f64::consts::PI * K_B * temperature / (G * mu_mh * mu_mh * number_density)).sqrt()
}

pub fn freefall_time(number_density: f64, mean_molecular_weight: f64) -> f64 {
    let rho = number_density * mean_molecular_weight * PROTON_MASS_KG;
    (3.0 * std::f64::consts::PI / (32.0 * G * rho)).sqrt()
}

pub fn cloud_thermal_velocity(temperature: f64, mean_molecular_weight: f64) -> f64 {
    (K_B * temperature / (mean_molecular_weight * PROTON_MASS_KG)).sqrt()
}

pub fn bonnor_ebert_mass(
    temperature: f64,
    external_pressure: f64,
    mean_molecular_weight: f64,
) -> f64 {
    let cs = cloud_thermal_velocity(temperature, mean_molecular_weight);
    1.182 * cs.powi(4) / (G.powi(3) * external_pressure).sqrt()
}

pub fn photodissociation_rate(
    unshielded_rate: f64,
    uv_field_habing: f64,
    shielding_factor: f64,
    visual_extinction: f64,
) -> f64 {
    unshielded_rate * uv_field_habing * (-shielding_factor * visual_extinction).exp()
}

pub fn thermal_desorption_rate(
    attempt_frequency: f64,
    binding_energy: f64,
    dust_temperature: f64,
) -> f64 {
    attempt_frequency * (-binding_energy / (K_B * dust_temperature)).exp()
}

pub fn h2_formation_rate_on_dust(
    sticking_coefficient: f64,
    grain_cross_section: f64,
    grain_density: f64,
    temperature: f64,
) -> f64 {
    let v_th = (8.0 * K_B * temperature / (std::f64::consts::PI * PROTON_MASS_KG)).sqrt();
    0.5 * sticking_coefficient * grain_density * grain_cross_section * v_th
}

pub fn saha_ionization_ratio(
    temperature: f64,
    electron_density: f64,
    ionization_energy: f64,
    partition_ratio: f64,
) -> f64 {
    let thermal_factor = 2.0
        * (2.0 * std::f64::consts::PI * ELECTRON_MASS_KG * K_B * temperature).powf(1.5)
        / H.powi(3);
    let boltzmann = (-ionization_energy / (K_B * temperature)).exp();
    partition_ratio * thermal_factor * boltzmann / electron_density
}

pub fn stroemgren_radius(
    ionizing_photon_rate: f64,
    hydrogen_density: f64,
    recombination_coeff: f64,
) -> f64 {
    (3.0 * ionizing_photon_rate
        / (4.0 * std::f64::consts::PI * hydrogen_density.powi(2) * recombination_coeff))
        .cbrt()
}

pub fn dust_equilibrium_temperature(
    luminosity: f64,
    distance: f64,
    absorption_efficiency: f64,
) -> f64 {
    (luminosity * absorption_efficiency
        / (16.0 * std::f64::consts::PI * distance.powi(2) * SIGMA_SB))
        .powf(0.25)
}
