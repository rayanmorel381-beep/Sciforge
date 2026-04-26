use std::f64::consts::PI;

use crate::constants::{
    ALPHA_FINE, ALPHA_S, C, E_CHARGE, G, G_F, H, HBAR, K_B, PLANCK_LENGTH, PLANCK_MASS,
    PLANCK_TEMP, PLANCK_TIME,
};

pub fn planck_energy() -> f64 {
    PLANCK_MASS * C * C
}

pub fn planck_density() -> f64 {
    PLANCK_MASS / (PLANCK_LENGTH * PLANCK_LENGTH * PLANCK_LENGTH)
}

pub fn planck_force() -> f64 {
    PLANCK_MASS * C * C / PLANCK_LENGTH
}

pub fn planck_pressure() -> f64 {
    planck_force() / (PLANCK_LENGTH * PLANCK_LENGTH)
}

pub fn planck_temperature() -> f64 {
    PLANCK_TEMP
}

pub fn planck_charge() -> f64 {
    E_CHARGE / ALPHA_FINE.sqrt()
}

pub fn planck_impedance() -> f64 {
    HBAR / (E_CHARGE * E_CHARGE)
}

pub fn planck_angular_frequency() -> f64 {
    1.0 / PLANCK_TIME
}

pub fn schwarzschild_radius_planck(mass_planck_units: f64) -> f64 {
    2.0 * mass_planck_units * PLANCK_LENGTH
}

pub fn hawking_temperature(mass: f64) -> f64 {
    HBAR * C * C * C / (8.0 * PI * G * mass * K_B)
}

pub fn hawking_luminosity(mass: f64) -> f64 {
    HBAR * C * C * C * C * C * C / (15360.0 * PI * G * G * mass * mass)
}

pub fn hawking_evaporation_time(mass: f64) -> f64 {
    5120.0 * PI * G * G * mass.powi(3) / (HBAR * C.powi(4))
}

pub fn unruh_temperature(acceleration: f64) -> f64 {
    HBAR * acceleration / (2.0 * PI * C * K_B)
}

pub fn fermi_coupling_constant() -> f64 {
    G_F
}

pub fn weak_decay_rate(energy_gev: f64) -> f64 {
    let energy_j = energy_gev * 1.602_176_634e-10;
    G_F * G_F * energy_j.powi(5) / (192.0 * PI * PI * PI * HBAR)
}

pub fn muon_decay_width() -> f64 {
    let m_mu_gev: f64 = 0.105_658_375_5;
    let m_mu_j = m_mu_gev * 1.602_176_634e-10;
    G_F * G_F * m_mu_j.powi(5) / (192.0 * PI.powi(3) * HBAR)
}

pub fn fine_structure_constant() -> f64 {
    ALPHA_FINE
}

pub fn strong_coupling_constant() -> f64 {
    ALPHA_S
}

pub fn qcd_running_coupling(q_gev: f64) -> f64 {
    let n_f = 6.0;
    let beta_0 = 11.0 - 2.0 * n_f / 3.0;
    let m_z_gev = 91.1876;
    let alpha_mz = ALPHA_S;
    alpha_mz / (1.0 + alpha_mz * beta_0 / (2.0 * PI) * (q_gev / m_z_gev).ln())
}

pub fn electromagnetic_coupling_running(q_gev: f64) -> f64 {
    let m_z_gev = 91.1876;
    ALPHA_FINE / (1.0 - ALPHA_FINE / (3.0 * PI) * (q_gev / m_z_gev).powi(2).ln())
}

pub fn weak_mixing_angle() -> f64 {
    0.231_16
}

pub fn w_boson_mass_gev() -> f64 {
    80.379
}

pub fn z_boson_mass_gev() -> f64 {
    91.187_6
}

pub fn higgs_vev_gev() -> f64 {
    1.0 / (2.0_f64.sqrt() * G_F * 1e-10_f64.powf(2.0)).powf(0.25)
}

pub fn compton_time(mass: f64) -> f64 {
    HBAR / (mass * C * C)
}

pub fn gravitational_coupling(m1: f64, m2: f64) -> f64 {
    G * m1 * m2 / (HBAR * C)
}

pub fn photon_energy(frequency: f64) -> f64 {
    H * frequency
}

pub fn photon_momentum(frequency: f64) -> f64 {
    H * frequency / C
}

pub fn pair_production_threshold_energy() -> f64 {
    2.0 * crate::constants::ELECTRON_MASS_KG * C * C
}

pub fn cross_section_thomson() -> f64 {
    let r_e = E_CHARGE * E_CHARGE
        / (4.0 * PI * crate::constants::EPSILON_0 * crate::constants::ELECTRON_MASS_KG * C * C);
    8.0 * PI * r_e * r_e / 3.0
}

pub fn neutrino_mass_upper_bound() -> f64 {
    crate::constants::NEUTRINO_MASS_UPPER
}

pub fn neutrino_de_broglie_wavelength(energy_ev: f64) -> f64 {
    let e_j = energy_ev * crate::constants::EV_TO_JOULE;
    H / (2.0 * crate::constants::ELECTRON_MASS_KG * e_j).sqrt()
}

pub fn classical_electron_radius() -> f64 {
    E_CHARGE * E_CHARGE
        / (4.0 * PI * crate::constants::EPSILON_0 * crate::constants::ELECTRON_MASS_KG * C * C)
}

pub fn bohr_velocity() -> f64 {
    ALPHA_FINE * C
}

pub fn schwinger_critical_field() -> f64 {
    crate::constants::ELECTRON_MASS_KG.powi(2) * C.powi(3) / (E_CHARGE * HBAR)
}
