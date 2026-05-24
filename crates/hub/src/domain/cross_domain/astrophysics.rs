//! Astrophysics: Schwarzschild radius, synchrotron radiation, Eddington
//! luminosity, and other stellar/galactic physics formulae.

use sciforge_lib::constants::physics::{
    C, E_CHARGE, ELECTRON_MASS_KG, EPSILON_0, G, H, HBAR, K_B, PROTON_MASS_KG,
};

pub fn schwarzschild_radius(mass: f64) -> f64 {
    2.0 * G * mass / (C * C)
}

pub fn eddington_luminosity(mass: f64) -> f64 {
    let r_e =
        E_CHARGE * E_CHARGE / (4.0 * std::f64::consts::PI * EPSILON_0 * ELECTRON_MASS_KG * C * C);
    let sigma_t = 8.0 * std::f64::consts::PI * r_e * r_e / 3.0;
    4.0 * std::f64::consts::PI * G * mass * PROTON_MASS_KG * C / sigma_t
}

pub fn chandrasekhar_mass(mu_e: f64) -> f64 {
    5.836 / (mu_e * mu_e) * (HBAR * C / G).powf(1.5) / (PROTON_MASS_KG * PROTON_MASS_KG)
}

pub fn virial_temperature(mass: f64, radius: f64, mean_molecular_weight: f64) -> f64 {
    G * mass * mean_molecular_weight * PROTON_MASS_KG / (2.0 * K_B * radius)
}

pub fn bondi_accretion_rate(mass: f64, density: f64, sound_speed: f64) -> f64 {
    4.0 * std::f64::consts::PI * G * G * mass * mass * density / sound_speed.powi(3)
}

pub fn compton_wavelength(mass: f64) -> f64 {
    H / (mass * C)
}

pub fn gravitational_redshift(mass: f64, radius: f64) -> f64 {
    1.0 / (1.0 - 2.0 * G * mass / (radius * C * C)).sqrt() - 1.0
}

pub fn synchrotron_critical_frequency(gamma_factor: f64, magnetic_field: f64) -> f64 {
    3.0 * E_CHARGE * magnetic_field * gamma_factor * gamma_factor
        / (4.0 * std::f64::consts::PI * ELECTRON_MASS_KG)
}

pub fn photon_sphere_radius(mass: f64) -> f64 {
    3.0 * G * mass / (C * C)
}

pub fn hawking_temperature(mass: f64) -> f64 {
    HBAR * C * C * C / (8.0 * std::f64::consts::PI * G * mass * K_B)
}

pub fn relativistic_doppler(frequency: f64, velocity: f64) -> f64 {
    frequency * ((1.0 - velocity / C) / (1.0 + velocity / C)).sqrt()
}
