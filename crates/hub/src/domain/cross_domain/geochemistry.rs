//! Geochemistry: partition coefficients, Nernst distribution,
//! radiogenic isotope ratios, and aqueous geochemical equilibria.

use sciforge_lib::constants::physics::{FARADAY, R_GAS};

pub fn partition_coefficient(c_solid: f64, c_liquid: f64) -> f64 {
    c_solid / c_liquid
}

pub fn batch_melting(c0: f64, melt_fraction: f64, bulk_d: f64) -> f64 {
    c0 / (bulk_d + melt_fraction * (1.0 - bulk_d))
}

pub fn fractional_crystallization(c0: f64, fraction_remaining: f64, bulk_d: f64) -> f64 {
    c0 * fraction_remaining.powf(bulk_d - 1.0)
}

pub fn delta_notation(r_sample: f64, r_standard: f64) -> f64 {
    (r_sample / r_standard - 1.0) * 1000.0
}

pub fn rayleigh_fractionation(r0: f64, fraction_remaining: f64, alpha: f64) -> f64 {
    r0 * fraction_remaining.powf(alpha - 1.0)
}

pub fn weathering_rate(rate_constant: f64, surface_area: f64, saturation_state: f64) -> f64 {
    rate_constant * surface_area * (1.0 - saturation_state)
}

pub fn activity_coefficient_debye_huckel(z: f64, ionic_strength: f64) -> f64 {
    let a = 0.509;
    let b = 0.328;
    let log_gamma = -a * z * z * ionic_strength.sqrt() / (1.0 + b * ionic_strength.sqrt());
    10.0_f64.powf(log_gamma)
}

pub fn solubility_product_temperature(ksp0: f64, delta_h: f64, t: f64, t0: f64) -> f64 {
    ksp0 * (-delta_h / R_GAS * (1.0 / t - 1.0 / t0)).exp()
}

pub fn eh_ph_boundary(e0: f64, n_electrons: f64, n_protons: f64, ph: f64, temperature: f64) -> f64 {
    e0 - (n_protons * R_GAS * temperature * 10.0_f64.ln()) / (n_electrons * FARADAY) * ph
}

pub fn distribution_coefficient(c_adsorbed: f64, c_solution: f64) -> f64 {
    c_adsorbed / c_solution
}

pub fn mixing_two_component(c1: f64, c2: f64, fraction_1: f64) -> f64 {
    fraction_1 * c1 + (1.0 - fraction_1) * c2
}
