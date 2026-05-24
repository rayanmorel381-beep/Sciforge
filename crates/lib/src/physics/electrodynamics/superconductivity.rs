use crate::constants::{E_CHARGE, H, HBAR, K_B, MU_0};
use std::f64::consts::PI;

pub fn bcs_gap_zero_temperature(tc_k: f64) -> f64 {
    1.764 * K_B * tc_k
}

pub fn bcs_gap_temperature(gap_zero_j: f64, t_k: f64, tc_k: f64) -> f64 {
    if t_k >= tc_k {
        0.0
    } else {
        gap_zero_j * (1.0 - (t_k / tc_k).powi(4)).sqrt()
    }
}

pub fn cooper_pair_binding_energy_j(gap_j: f64) -> f64 {
    2.0 * gap_j
}

pub fn flux_quantum() -> f64 {
    H / (2.0 * E_CHARGE)
}

pub fn london_penetration_depth(
    superfluid_density_per_m3: f64,
    effective_mass_kg: f64,
) -> f64 {
    let m = effective_mass_kg;
    (m / (MU_0 * superfluid_density_per_m3 * E_CHARGE * E_CHARGE)).sqrt()
}

pub fn london_depth_temperature(lambda_0_m: f64, t_k: f64, tc_k: f64) -> f64 {
    if t_k >= tc_k {
        f64::INFINITY
    } else {
        lambda_0_m / (1.0 - (t_k / tc_k).powi(4)).sqrt()
    }
}

pub fn coherence_length_pippard(fermi_velocity_m_per_s: f64, gap_j: f64) -> f64 {
    HBAR * fermi_velocity_m_per_s / (PI * gap_j)
}

pub fn ginzburg_landau_kappa(lambda_m: f64, xi_m: f64) -> f64 {
    lambda_m / xi_m
}

pub fn type_classification(kappa: f64) -> &'static str {
    if kappa < 1.0 / std::f64::consts::SQRT_2 {
        "Type I"
    } else {
        "Type II"
    }
}

pub fn critical_field_thermodynamic(
    bc0_t: f64,
    t_k: f64,
    tc_k: f64,
) -> f64 {
    if t_k >= tc_k {
        0.0
    } else {
        bc0_t * (1.0 - (t_k / tc_k).powi(2))
    }
}

pub fn lower_critical_field(bc_t: f64, kappa: f64) -> f64 {
    bc_t * kappa.ln() / (kappa * std::f64::consts::SQRT_2)
}

pub fn upper_critical_field(bc_t: f64, kappa: f64) -> f64 {
    bc_t * kappa * std::f64::consts::SQRT_2
}

pub fn bc2_from_xi(xi_m: f64) -> f64 {
    flux_quantum() / (2.0 * PI * xi_m * xi_m)
}

pub fn josephson_dc_current(critical_current_a: f64, phase_rad: f64) -> f64 {
    critical_current_a * phase_rad.sin()
}

pub fn josephson_ac_voltage_to_frequency(voltage_v: f64) -> f64 {
    2.0 * E_CHARGE * voltage_v / H
}

pub fn shapiro_step_voltage(frequency_hz: f64, n: u32) -> f64 {
    n as f64 * H * frequency_hz / (2.0 * E_CHARGE)
}

pub fn squid_max_voltage_modulation(critical_current_a: f64, normal_resistance_ohm: f64) -> f64 {
    critical_current_a * normal_resistance_ohm
}

pub fn ambegaokar_baratoff_ic(gap_j: f64, normal_resistance_ohm: f64, t_k: f64) -> f64 {
    let kt = K_B * t_k;
    PI * gap_j / (2.0 * E_CHARGE * normal_resistance_ohm) * (gap_j / (2.0 * kt)).tanh()
}

pub fn bcs_isotope_effect_tc(tc_ref_k: f64, mass_ref_amu: f64, mass_amu: f64, alpha: f64) -> f64 {
    tc_ref_k * (mass_ref_amu / mass_amu).powf(alpha)
}

pub fn condensation_energy_density(
    bc_t: f64,
) -> f64 {
    bc_t * bc_t / (2.0 * MU_0)
}

pub fn specific_heat_jump_bcs(gamma_n_normal: f64, tc_k: f64) -> f64 {
    1.43 * gamma_n_normal * tc_k
}
