use crate::constants::{E_CHARGE, ELECTRON_MASS_KG, K_B};

pub fn drude_conductivity(carrier_density_per_m3: f64, relaxation_time_s: f64) -> f64 {
    carrier_density_per_m3 * E_CHARGE * E_CHARGE * relaxation_time_s / ELECTRON_MASS_KG
}

pub fn drude_resistivity(carrier_density_per_m3: f64, relaxation_time_s: f64) -> f64 {
    1.0 / drude_conductivity(carrier_density_per_m3, relaxation_time_s)
}

pub fn drude_mobility(relaxation_time_s: f64, effective_mass_kg: f64) -> f64 {
    E_CHARGE * relaxation_time_s / effective_mass_kg
}

pub fn wiedemann_franz_thermal_conductivity(electrical_conductivity_s_per_m: f64, t_k: f64) -> f64 {
    let lorenz = std::f64::consts::PI.powi(2) / 3.0 * (K_B / E_CHARGE).powi(2);
    lorenz * electrical_conductivity_s_per_m * t_k
}

pub fn lorenz_number() -> f64 {
    std::f64::consts::PI.powi(2) / 3.0 * (K_B / E_CHARGE).powi(2)
}

pub fn matthiessen_rule(rho_phonon: f64, rho_impurity: f64) -> f64 {
    rho_phonon + rho_impurity
}

pub fn matthiessen_n(resistivities: &[f64]) -> f64 {
    resistivities.iter().sum()
}

pub fn resistivity_temperature(rho_0: f64, alpha_per_k: f64, t_k: f64, t_ref_k: f64) -> f64 {
    rho_0 * (1.0 + alpha_per_k * (t_k - t_ref_k))
}

pub fn bloch_grueneisen_resistivity(
    rho_0: f64,
    a_coeff: f64,
    t_k: f64,
    debye_temp_k: f64,
) -> f64 {
    let n = 5;
    let ratio = t_k / debye_temp_k;
    let mut sum = 0.0;
    let steps = 200;
    let dx = 1.0 / steps as f64;
    for i in 1..=steps {
        let x = (i as f64 - 0.5) * dx;
        let denom = (x.exp() - 1.0) * (1.0 - (-x).exp());
        sum += x.powi(n) / denom * dx;
    }
    rho_0 + a_coeff * ratio.powi(n) * sum
}

pub fn fermi_velocity(fermi_energy_j: f64, mass_kg: f64) -> f64 {
    (2.0 * fermi_energy_j / mass_kg).sqrt()
}

pub fn mean_free_path(fermi_velocity_m_per_s: f64, relaxation_time_s: f64) -> f64 {
    fermi_velocity_m_per_s * relaxation_time_s
}

pub fn relaxation_time_from_resistivity(
    resistivity_ohm_m: f64,
    carrier_density_per_m3: f64,
) -> f64 {
    ELECTRON_MASS_KG / (carrier_density_per_m3 * E_CHARGE * E_CHARGE * resistivity_ohm_m)
}

pub fn hall_mobility(hall_coefficient_m3_per_c: f64, conductivity_s_per_m: f64) -> f64 {
    hall_coefficient_m3_per_c.abs() * conductivity_s_per_m
}

pub fn skin_depth(resistivity_ohm_m: f64, frequency_hz: f64, mu_relative: f64) -> f64 {
    let omega = 2.0 * std::f64::consts::PI * frequency_hz;
    (2.0 * resistivity_ohm_m / (omega * mu_relative * crate::constants::MU_0)).sqrt()
}

pub fn anderson_localization_criterion(disorder_w: f64, hopping_t: f64) -> f64 {
    disorder_w / hopping_t
}
