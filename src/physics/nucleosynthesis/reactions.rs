use super::nuclide::{AMU_TO_MEV, BOLTZMANN_KEV, Nuclide};
use crate::constants::{BARN, FERMI, MEV_TO_J};

pub fn q_value(reactants: &[&Nuclide], products: &[&Nuclide]) -> f64 {
    let mass_in: f64 = reactants.iter().map(|n| n.atomic_mass_amu()).sum();
    let mass_out: f64 = products.iter().map(|n| n.atomic_mass_amu()).sum();
    (mass_in - mass_out) * AMU_TO_MEV
}

pub fn coulomb_barrier(z1: u32, z2: u32, a1: u32, a2: u32) -> f64 {
    let e2 = 1.44;
    let r0 = 1.2;
    let r = r0 * ((a1 as f64).powf(1.0 / 3.0) + (a2 as f64).powf(1.0 / 3.0));
    e2 * z1 as f64 * z2 as f64 / r
}

pub fn gamow_peak(z1: u32, z2: u32, reduced_mass_amu: f64, temperature_k: f64) -> f64 {
    let kt = BOLTZMANN_KEV * temperature_k * 1e-3;
    let eg = 0.978 * (z1 as f64 * z2 as f64).powi(2) * reduced_mass_amu;
    (eg * kt * kt / 4.0).powf(1.0 / 3.0)
}

pub fn gamow_window_width(z1: u32, z2: u32, reduced_mass_amu: f64, temperature_k: f64) -> f64 {
    let e0 = gamow_peak(z1, z2, reduced_mass_amu, temperature_k);
    let kt = BOLTZMANN_KEV * temperature_k * 1e-3;
    4.0 * (e0 * kt / 3.0).sqrt()
}

pub fn reduced_mass_amu(m1: f64, m2: f64) -> f64 {
    m1 * m2 / (m1 + m2)
}

pub fn astrophysical_s_factor(
    cross_section_barn: f64,
    energy_kev: f64,
    z1: u32,
    z2: u32,
    mu_amu: f64,
) -> f64 {
    let eta = 0.1575 * z1 as f64 * z2 as f64 * (mu_amu / energy_kev).sqrt();
    let sommerfeld = (-2.0 * std::f64::consts::PI * eta).exp();
    if sommerfeld < 1e-300 {
        return 0.0;
    }
    cross_section_barn * energy_kev / sommerfeld
}

pub fn sommerfeld_parameter(z1: u32, z2: u32, energy_kev: f64, mu_amu: f64) -> f64 {
    0.1575 * z1 as f64 * z2 as f64 * (mu_amu / energy_kev).sqrt()
}

pub fn penetration_factor(z1: u32, z2: u32, energy_kev: f64, mu_amu: f64) -> f64 {
    let eta = sommerfeld_parameter(z1, z2, energy_kev, mu_amu);
    let two_pi_eta = 2.0 * std::f64::consts::PI * eta;
    if two_pi_eta > 700.0 {
        return 0.0;
    }
    two_pi_eta / (two_pi_eta.exp() - 1.0)
}

pub fn thermonuclear_rate(
    s_factor_kev_barn: f64,
    z1: u32,
    z2: u32,
    mu_amu: f64,
    temperature_k: f64,
) -> f64 {
    let kt = BOLTZMANN_KEV * temperature_k;
    let t9 = temperature_k / 1e9;
    let tau = 42.487 * (z1 as f64 * z2 as f64).powi(2) * mu_amu / t9;
    let tau_third = tau.powf(1.0 / 3.0);
    let prefactor = 7.831e9 / (mu_amu * t9 * t9).powf(1.0 / 6.0);
    let _ = kt;
    prefactor * s_factor_kev_barn * (-3.0 * tau_third).exp()
}

pub fn pp_rate_estimate(temperature_k: f64, density_g_cm3: f64, x_h: f64) -> f64 {
    let t9 = temperature_k / 1e9;
    let t9_13 = t9.powf(1.0 / 3.0);
    3.38e-18 * density_g_cm3 * x_h * x_h * t9.powf(-2.0 / 3.0) * (-3.381 / t9_13).exp()
}

pub fn triple_alpha_rate_estimate(temperature_k: f64, density_g_cm3: f64, y_he: f64) -> f64 {
    let t9 = temperature_k / 1e9;
    5.09e8 * density_g_cm3 * density_g_cm3 * y_he.powi(3) * t9.powf(-3.0) * (-4.4027 / t9).exp()
}

pub fn c12_alpha_rate_estimate(temperature_k: f64) -> f64 {
    let t9 = temperature_k / 1e9;
    let t9_13 = t9.powf(1.0 / 3.0);
    1.04e8 * t9.powf(-2.0 / 3.0) * (-32.12 / t9_13 - (t9 / 3.496).powi(2)).exp()
        + 1.76e8 * t9.powf(-2.0 / 3.0) * (-32.12 / t9_13).exp()
}

pub fn reaction_mean_free_path(cross_section_barn: f64, number_density_per_cm3: f64) -> f64 {
    1.0 / (cross_section_barn * 1e-24 * number_density_per_cm3)
}

pub fn nuclear_reaction_lifetime(
    cross_section_barn: f64,
    number_density_per_cm3: f64,
    velocity_cm_s: f64,
) -> f64 {
    1.0 / (cross_section_barn * 1e-24 * number_density_per_cm3 * velocity_cm_s)
}

pub fn maxwell_averaged_velocity(temperature_k: f64, mu_amu: f64) -> f64 {
    let mu_kg = mu_amu * crate::constants::AMU_TO_KG;
    (8.0 * crate::constants::K_B * temperature_k / (std::f64::consts::PI * mu_kg)).sqrt()
}

pub fn cross_section_barn_to_si(sigma_barn: f64) -> f64 {
    sigma_barn * BARN
}

pub fn nuclear_radius_fermi(a: u32) -> f64 {
    1.2 * FERMI * (a as f64).powf(1.0 / 3.0)
}

pub fn nuclear_volume(a: u32) -> f64 {
    let r = nuclear_radius_fermi(a);
    4.0 / 3.0 * std::f64::consts::PI * r.powi(3)
}

pub fn q_value_to_joules(q_mev: f64) -> f64 {
    q_mev * MEV_TO_J
}

pub fn geometric_cross_section(a1: u32, a2: u32) -> f64 {
    let r1 = 1.2 * FERMI * (a1 as f64).powf(1.0 / 3.0);
    let r2 = 1.2 * FERMI * (a2 as f64).powf(1.0 / 3.0);
    std::f64::consts::PI * (r1 + r2).powi(2)
}

pub fn geometric_cross_section_barn(a1: u32, a2: u32) -> f64 {
    geometric_cross_section(a1, a2) / BARN
}
