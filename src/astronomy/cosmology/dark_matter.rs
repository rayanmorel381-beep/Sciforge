use crate::constants::{
    AGE_UNIVERSE, AXION_DECAY_CONSTANT, AXION_MASS_MAX_EV, AXION_MASS_MIN_EV, C, CMB_TEMPERATURE,
    COSMOLOGICAL_LAMBDA, ELECTRON_MASS_KG, EV_TO_JOULE, G, HBAR, HUBBLE_CONSTANT, K_B,
    LOCAL_DM_DENSITY, LOCAL_DM_DENSITY_SI, MPC_IN_KM, MW_CONCENTRATION, MW_VIRIAL_MASS,
    MW_VIRIAL_RADIUS, NFW_RHO_S_TYPICAL, NFW_RS_MW, OMEGA_BARYON, OMEGA_DM, OMEGA_LAMBDA,
    OMEGA_MATTER, PARSEC, SEC_PER_GYR, SOLAR_MASS, THERMAL_RELIC_SIGMA_V, WIMP_MASS_MAX_GEV,
    WIMP_MASS_MIN_GEV, WIMP_SIGMA_UPPER, Z_MATTER_RADIATION_EQ, Z_RECOMBINATION,
};
use std::f64::consts::PI;

pub fn nfw_density_profile(r_kpc: f64) -> f64 {
    let x = r_kpc / NFW_RS_MW;
    NFW_RHO_S_TYPICAL * SOLAR_MASS / (x * (1.0 + x) * (1.0 + x))
}

pub fn nfw_enclosed_mass(r_kpc: f64) -> f64 {
    let x = r_kpc / NFW_RS_MW;
    4.0 * PI * NFW_RHO_S_TYPICAL * SOLAR_MASS * NFW_RS_MW.powi(3) * ((1.0 + x).ln() - x / (1.0 + x))
}

pub fn nfw_circular_velocity(r_kpc: f64) -> f64 {
    let r_m = r_kpc * 1e3 * PARSEC;
    let m_enc = nfw_enclosed_mass(r_kpc);
    (G * m_enc / r_m).sqrt()
}

pub fn nfw_concentration() -> f64 {
    MW_CONCENTRATION
}

pub fn nfw_virial_mass() -> f64 {
    MW_VIRIAL_MASS * SOLAR_MASS
}

pub fn nfw_virial_radius() -> f64 {
    MW_VIRIAL_RADIUS
}

pub fn wimp_annihilation_rate(rho: f64, mass_gev: f64) -> f64 {
    let mass_kg = mass_gev * 1.782_662e-27;
    let n = rho / mass_kg;
    0.5 * n * n * THERMAL_RELIC_SIGMA_V
}

pub fn wimp_direct_detection_rate(rho: f64, mass_gev: f64, sigma_cm2: f64, v_mean: f64) -> f64 {
    let mass_kg = mass_gev * 1.782_662e-27;
    let n = rho / mass_kg;
    n * sigma_cm2 * v_mean
}

pub fn local_dm_density_gev() -> f64 {
    LOCAL_DM_DENSITY
}

pub fn local_dm_density_si() -> f64 {
    LOCAL_DM_DENSITY_SI
}

pub fn wimp_mass_range() -> (f64, f64) {
    (WIMP_MASS_MIN_GEV, WIMP_MASS_MAX_GEV)
}

pub fn wimp_cross_section_upper() -> f64 {
    WIMP_SIGMA_UPPER
}

pub fn thermal_relic_cross_section() -> f64 {
    THERMAL_RELIC_SIGMA_V
}

pub fn axion_compton_frequency(mass_ev: f64) -> f64 {
    mass_ev * EV_TO_JOULE / HBAR
}

pub fn axion_mass_range() -> (f64, f64) {
    (AXION_MASS_MIN_EV, AXION_MASS_MAX_EV)
}

pub fn axion_decay_constant() -> f64 {
    AXION_DECAY_CONSTANT
}

pub fn axion_de_broglie_wavelength(mass_ev: f64, velocity: f64) -> f64 {
    let mass_kg = mass_ev * EV_TO_JOULE / (C * C);
    HBAR / (mass_kg * velocity)
}

pub fn dm_relic_density() -> f64 {
    OMEGA_DM
}

pub fn baryon_density() -> f64 {
    OMEGA_BARYON
}

pub fn matter_density() -> f64 {
    OMEGA_MATTER
}

pub fn dark_energy_density_today() -> f64 {
    OMEGA_LAMBDA
}

pub fn matter_radiation_equality_temperature() -> f64 {
    CMB_TEMPERATURE * (1.0 + Z_MATTER_RADIATION_EQ)
}

pub fn matter_radiation_equality_redshift() -> f64 {
    Z_MATTER_RADIATION_EQ
}

pub fn recombination_temperature() -> f64 {
    CMB_TEMPERATURE * (1.0 + Z_RECOMBINATION)
}

pub fn recombination_redshift() -> f64 {
    Z_RECOMBINATION
}

pub fn age_of_universe_seconds() -> f64 {
    AGE_UNIVERSE
}

pub fn baryon_to_dm_ratio() -> f64 {
    OMEGA_BARYON / OMEGA_DM
}

pub fn dm_annihilation_luminosity(rho: f64, mass_gev: f64, volume: f64) -> f64 {
    let mass_kg = mass_gev * 1.782_662e-27;
    let n = rho / mass_kg;
    let rate = 0.5 * n * n * THERMAL_RELIC_SIGMA_V;
    rate * mass_kg * C * C * volume
}

pub fn saha_ionization_fraction(temperature: f64, n_b: f64) -> f64 {
    let binding_energy = 13.6 * EV_TO_JOULE;
    let me = ELECTRON_MASS_KG;
    let lambda_th = HBAR * (2.0 * PI / (me * K_B * temperature)).sqrt();
    let rhs = (1.0 / (n_b * lambda_th.powi(3))) * (-binding_energy / (K_B * temperature)).exp();
    let discriminant = (rhs * rhs + 4.0 * rhs).sqrt();
    (-rhs + discriminant) / 2.0
}

pub fn hubble_constant() -> f64 {
    HUBBLE_CONSTANT
}

pub fn cosmological_constant() -> f64 {
    COSMOLOGICAL_LAMBDA
}

pub fn hubble_time() -> f64 {
    1.0 / (HUBBLE_CONSTANT / MPC_IN_KM) / SEC_PER_GYR
}

pub fn hubble_radius() -> f64 {
    C / (HUBBLE_CONSTANT / MPC_IN_KM)
}
