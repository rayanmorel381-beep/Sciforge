use crate::constants::{E_CHARGE, K_B};

pub fn varshni_bandgap_ev(eg0_ev: f64, alpha_ev_per_k: f64, beta_k: f64, t_k: f64) -> f64 {
    eg0_ev - alpha_ev_per_k * t_k * t_k / (t_k + beta_k)
}

pub fn intrinsic_carrier_density(nc_per_m3: f64, nv_per_m3: f64, eg_ev: f64, t_k: f64) -> f64 {
    let kt_ev = K_B * t_k / E_CHARGE;
    (nc_per_m3 * nv_per_m3).sqrt() * (-eg_ev / (2.0 * kt_ev)).exp()
}

pub fn effective_dos_conduction(m_eff_ratio: f64, t_k: f64) -> f64 {
    let m_e_kg = crate::constants::ELECTRON_MASS_KG;
    let h = crate::constants::H;
    2.0 * (2.0 * std::f64::consts::PI * m_eff_ratio * m_e_kg * K_B * t_k / (h * h)).powf(1.5)
}

pub fn mass_action_law(n_per_m3: f64, p_per_m3: f64) -> f64 {
    n_per_m3 * p_per_m3
}

pub fn fermi_level_intrinsic_ev(eg_ev: f64, nc_per_m3: f64, nv_per_m3: f64, t_k: f64) -> f64 {
    let kt_ev = K_B * t_k / E_CHARGE;
    eg_ev / 2.0 + 0.5 * kt_ev * (nv_per_m3 / nc_per_m3).ln()
}

pub fn shockley_diode_current(
    is_a: f64,
    voltage_v: f64,
    ideality: f64,
    t_k: f64,
) -> f64 {
    let vt = K_B * t_k / E_CHARGE;
    is_a * ((voltage_v / (ideality * vt)).exp() - 1.0)
}

pub fn thermal_voltage(t_k: f64) -> f64 {
    K_B * t_k / E_CHARGE
}

pub fn debye_length_semiconductor(epsilon_r: f64, doping_per_m3: f64, t_k: f64) -> f64 {
    let eps = epsilon_r * crate::constants::EPSILON_0;
    (eps * K_B * t_k / (doping_per_m3 * E_CHARGE * E_CHARGE)).sqrt()
}

pub fn built_in_voltage(na_per_m3: f64, nd_per_m3: f64, ni_per_m3: f64, t_k: f64) -> f64 {
    let vt = K_B * t_k / E_CHARGE;
    vt * (na_per_m3 * nd_per_m3 / (ni_per_m3 * ni_per_m3)).ln()
}

pub fn depletion_width(
    epsilon_r: f64,
    v_bi_v: f64,
    v_applied_v: f64,
    na_per_m3: f64,
    nd_per_m3: f64,
) -> f64 {
    let eps = epsilon_r * crate::constants::EPSILON_0;
    let inv_n = 1.0 / na_per_m3 + 1.0 / nd_per_m3;
    (2.0 * eps * (v_bi_v - v_applied_v) * inv_n / E_CHARGE).sqrt()
}

pub fn drift_current_density(charge_density_per_m3: f64, mobility_m2_per_vs: f64, e_field_v_per_m: f64) -> f64 {
    E_CHARGE * charge_density_per_m3 * mobility_m2_per_vs * e_field_v_per_m
}

pub fn diffusion_current_density(diffusion_m2_per_s: f64, gradient_per_m4: f64) -> f64 {
    -E_CHARGE * diffusion_m2_per_s * gradient_per_m4
}

pub fn einstein_diffusion(mobility_m2_per_vs: f64, t_k: f64) -> f64 {
    mobility_m2_per_vs * K_B * t_k / E_CHARGE
}

pub fn hall_coefficient(carrier_density_per_m3: f64, sign: f64) -> f64 {
    sign / (carrier_density_per_m3 * E_CHARGE)
}

pub fn conductivity_semiconductor(
    n_per_m3: f64,
    mu_n: f64,
    p_per_m3: f64,
    mu_p: f64,
) -> f64 {
    E_CHARGE * (n_per_m3 * mu_n + p_per_m3 * mu_p)
}

pub fn fermi_energy_free_electron(carrier_density_per_m3: f64, effective_mass_kg: f64) -> f64 {
    let hbar = crate::constants::HBAR;
    hbar * hbar / (2.0 * effective_mass_kg)
        * (3.0 * std::f64::consts::PI * std::f64::consts::PI * carrier_density_per_m3)
            .powf(2.0 / 3.0)
}

pub fn fermi_dirac_distribution(energy_j: f64, fermi_energy_j: f64, t_k: f64) -> f64 {
    1.0 / (((energy_j - fermi_energy_j) / (K_B * t_k)).exp() + 1.0)
}

pub fn drift_velocity(mobility_m2_per_vs: f64, e_field_v_per_m: f64) -> f64 {
    mobility_m2_per_vs * e_field_v_per_m
}

pub fn doping_ionization_fraction(
    donor_density_per_m3: f64,
    donor_level_j: f64,
    t_k: f64,
) -> f64 {
    donor_density_per_m3 / (1.0 + 0.5 * (donor_level_j / (K_B * t_k)).exp())
}

pub fn hall_coefficient_two_carrier(
    n_per_m3: f64,
    p_per_m3: f64,
    mu_n: f64,
    mu_p: f64,
) -> f64 {
    let num = p_per_m3 * mu_p * mu_p - n_per_m3 * mu_n * mu_n;
    let den = E_CHARGE * (p_per_m3 * mu_p + n_per_m3 * mu_n).powi(2);
    num / den.max(1e-50)
}

pub fn depletion_width_unbiased(
    epsilon_pa_per_m: f64,
    built_in_voltage_v: f64,
    na_per_m3: f64,
    nd_per_m3: f64,
) -> f64 {
    (2.0 * epsilon_pa_per_m * built_in_voltage_v * (na_per_m3 + nd_per_m3)
        / (E_CHARGE * na_per_m3 * nd_per_m3))
        .sqrt()
}
