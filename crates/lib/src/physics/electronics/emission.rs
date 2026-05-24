use crate::constants::{E_CHARGE, ELECTRON_MASS_KG, H, K_B};
use std::f64::consts::PI;

pub fn richardson_constant() -> f64 {
    4.0 * PI * ELECTRON_MASS_KG * E_CHARGE * K_B * K_B / H.powi(3)
}

pub fn richardson_dushman_current_density(work_function_ev: f64, t_k: f64) -> f64 {
    let phi_j = work_function_ev * E_CHARGE;
    richardson_constant() * t_k * t_k * (-phi_j / (K_B * t_k)).exp()
}

pub fn schottky_effective_work_function(
    work_function_ev: f64,
    e_field_v_per_m: f64,
) -> f64 {
    let delta_phi_j = (E_CHARGE.powi(3) * e_field_v_per_m
        / (4.0 * PI * crate::constants::EPSILON_0))
        .sqrt();
    work_function_ev - delta_phi_j / E_CHARGE
}

pub fn schottky_current_density(
    work_function_ev: f64,
    t_k: f64,
    e_field_v_per_m: f64,
) -> f64 {
    let phi_eff_ev = schottky_effective_work_function(work_function_ev, e_field_v_per_m);
    richardson_dushman_current_density(phi_eff_ev, t_k)
}

pub fn fowler_nordheim_current_density(
    work_function_ev: f64,
    e_field_v_per_m: f64,
) -> f64 {
    let phi_j = work_function_ev * E_CHARGE;
    let a_fn = E_CHARGE.powi(3) / (16.0 * PI * PI * crate::constants::HBAR * phi_j);
    let b_fn = 4.0 / 3.0 * (2.0 * ELECTRON_MASS_KG).sqrt() * phi_j.powf(1.5)
        / (E_CHARGE * crate::constants::HBAR);
    a_fn * e_field_v_per_m * e_field_v_per_m * (-b_fn / e_field_v_per_m).exp()
}

pub fn photoelectric_threshold_wavelength(work_function_ev: f64) -> f64 {
    let phi_j = work_function_ev * E_CHARGE;
    H * crate::constants::C / phi_j
}

pub fn photoelectron_kinetic_energy_ev(photon_energy_ev: f64, work_function_ev: f64) -> f64 {
    (photon_energy_ev - work_function_ev).max(0.0)
}

pub fn fowler_photoemission_yield(photon_energy_ev: f64, work_function_ev: f64, t_k: f64) -> f64 {
    let kt_ev = K_B * t_k / E_CHARGE;
    let mu = (photon_energy_ev - work_function_ev) / kt_ev;
    if mu <= 0.0 {
        let exp_mu = mu.exp();
        exp_mu - exp_mu.powi(2) / 4.0 + exp_mu.powi(3) / 9.0
    } else {
        let pi2_6 = PI * PI / 6.0;
        pi2_6 + mu * mu / 2.0 - (-mu).exp() + (-2.0 * mu).exp() / 4.0
    }
}

pub fn schottky_barrier_height(work_function_metal_ev: f64, electron_affinity_ev: f64) -> f64 {
    work_function_metal_ev - electron_affinity_ev
}

pub fn contact_potential(work_function_a_ev: f64, work_function_b_ev: f64) -> f64 {
    work_function_a_ev - work_function_b_ev
}

pub fn child_langmuir_current_density(
    voltage_v: f64,
    distance_m: f64,
) -> f64 {
    let factor = 4.0 / 9.0 * crate::constants::EPSILON_0
        * (2.0 * E_CHARGE / ELECTRON_MASS_KG).sqrt();
    factor * voltage_v.powf(1.5) / (distance_m * distance_m)
}

pub fn thermionic_emission_efficiency(
    work_function_ev: f64,
    t_k: f64,
    voltage_v: f64,
) -> f64 {
    let phi = work_function_ev * E_CHARGE;
    let ev = voltage_v * E_CHARGE;
    let kt = K_B * t_k;
    ev / (phi + 2.0 * kt + ev)
}
