use crate::constants::{K_B, MU_0};
use std::f64::consts::PI;

pub fn curie_law(curie_constant_k_a_per_m_per_t: f64, t_k: f64) -> f64 {
    curie_constant_k_a_per_m_per_t / t_k
}

pub fn curie_weiss_law(curie_constant: f64, t_k: f64, theta_k: f64) -> f64 {
    curie_constant / (t_k - theta_k)
}

pub fn curie_constant(density_per_m3: f64, magnetic_moment_j_per_t: f64) -> f64 {
    MU_0 * density_per_m3 * magnetic_moment_j_per_t.powi(2) / (3.0 * K_B)
}

pub fn langevin_function(x: f64) -> f64 {
    if x.abs() < 1e-6 {
        x / 3.0 - x.powi(3) / 45.0
    } else {
        1.0 / x.tanh() - 1.0 / x
    }
}

pub fn langevin_paramagnetic_magnetization(
    saturation_a_per_m: f64,
    moment_j_per_t: f64,
    b_t: f64,
    t_k: f64,
) -> f64 {
    let x = moment_j_per_t * b_t / (K_B * t_k);
    saturation_a_per_m * langevin_function(x)
}

pub fn brillouin_function(j_quantum: f64, x: f64) -> f64 {
    if x.abs() < 1e-6 {
        (j_quantum + 1.0) * x / (3.0)
    } else {
        let a = (2.0 * j_quantum + 1.0) / (2.0 * j_quantum);
        let b = 1.0 / (2.0 * j_quantum);
        a / (a * x).tanh() - b / (b * x).tanh()
    }
}

pub fn steinmetz_hysteresis_loss(
    eta: f64,
    b_max_t: f64,
    n_exponent: f64,
    frequency_hz: f64,
) -> f64 {
    eta * b_max_t.powf(n_exponent) * frequency_hz
}

pub fn eddy_current_loss(
    k_eddy: f64,
    b_max_t: f64,
    frequency_hz: f64,
    thickness_m: f64,
) -> f64 {
    k_eddy * (b_max_t * frequency_hz * thickness_m).powi(2)
}

pub fn total_iron_loss(
    eta_steinmetz: f64,
    n_exponent: f64,
    k_eddy: f64,
    b_max_t: f64,
    frequency_hz: f64,
    thickness_m: f64,
) -> f64 {
    steinmetz_hysteresis_loss(eta_steinmetz, b_max_t, n_exponent, frequency_hz)
        + eddy_current_loss(k_eddy, b_max_t, frequency_hz, thickness_m)
}

pub fn magnetic_susceptibility_from_permeability(mu_r: f64) -> f64 {
    mu_r - 1.0
}

pub fn magnetization_from_susceptibility(chi_m: f64, h_a_per_m: f64) -> f64 {
    chi_m * h_a_per_m
}

pub fn magnetic_flux_density(mu_r: f64, h_a_per_m: f64) -> f64 {
    MU_0 * mu_r * h_a_per_m
}

pub fn energy_density_magnetic(mu_r: f64, h_a_per_m: f64) -> f64 {
    0.5 * MU_0 * mu_r * h_a_per_m * h_a_per_m
}

pub fn demagnetizing_field(n_factor: f64, magnetization_a_per_m: f64) -> f64 {
    -n_factor * magnetization_a_per_m
}

pub fn anisotropy_field(k_anisotropy_j_per_m3: f64, saturation_a_per_m: f64) -> f64 {
    2.0 * k_anisotropy_j_per_m3 / (MU_0 * saturation_a_per_m)
}

pub fn larmor_precession_frequency(b_t: f64, gyromagnetic_ratio: f64) -> f64 {
    gyromagnetic_ratio * b_t / (2.0 * PI)
}

pub fn bloch_t32_law(saturation_0_a_per_m: f64, beta: f64, t_k: f64) -> f64 {
    saturation_0_a_per_m * (1.0 - beta * t_k.powf(1.5))
}

pub fn pauli_paramagnetic_susceptibility(dos_at_fermi_per_j_per_m3: f64) -> f64 {
    let mu_b = 9.274_010_078_3e-24;
    MU_0 * mu_b * mu_b * dos_at_fermi_per_j_per_m3
}
