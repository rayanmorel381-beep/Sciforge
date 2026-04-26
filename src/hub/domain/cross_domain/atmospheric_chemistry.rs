//! Atmospheric chemistry: photolysis rates, ozone equilibrium, and
//! gas-phase reaction kinetics in planetary atmospheres.

use crate::constants::physics::{K_B, R_GAS};

pub fn photolysis_rate(cross_section: f64, quantum_yield: f64, actinic_flux: f64) -> f64 {
    cross_section * quantum_yield * actinic_flux
}

pub fn reaction_rate_arrhenius(prefactor: f64, activation_energy: f64, temperature: f64) -> f64 {
    prefactor * (-activation_energy / (R_GAS * temperature)).exp()
}

pub fn henry_law_concentration(henry_constant: f64, partial_pressure: f64) -> f64 {
    henry_constant * partial_pressure
}

pub fn chemical_lifetime(rate_constant: f64, co_reactant_density: f64) -> f64 {
    1.0 / (rate_constant * co_reactant_density)
}

pub fn mixing_ratio_to_number_density(mixing_ratio: f64, pressure: f64, temperature: f64) -> f64 {
    mixing_ratio * pressure / (K_B * temperature)
}

pub fn deposition_velocity(
    aerodynamic_resistance: f64,
    surface_resistance: f64,
    gravitational_settling: f64,
) -> f64 {
    1.0 / (aerodynamic_resistance + surface_resistance) + gravitational_settling
}

pub fn aerosol_optical_depth(extinction_coeff: f64, layer_thickness: f64) -> f64 {
    extinction_coeff * layer_thickness
}

pub fn equilibrium_constant_temperature(
    k_ref: f64,
    delta_h: f64,
    t_ref: f64,
    temperature: f64,
) -> f64 {
    k_ref * (-delta_h / R_GAS * (1.0 / temperature - 1.0 / t_ref)).exp()
}

pub fn lindemann_rate(k0: f64, kinf: f64, m_density: f64) -> f64 {
    k0 * m_density / (1.0 + k0 * m_density / kinf)
}

pub fn mean_free_path(temperature: f64, pressure: f64, collision_diameter: f64) -> f64 {
    K_B * temperature
        / (std::f64::consts::SQRT_2
            * std::f64::consts::PI
            * collision_diameter
            * collision_diameter
            * pressure)
}

pub fn column_density(number_density: f64, path_length: f64) -> f64 {
    number_density * path_length
}
