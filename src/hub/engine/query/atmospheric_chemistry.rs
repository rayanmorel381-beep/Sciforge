//! Catalog entries for atmospheric chemistry functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::AtmosphericChemistry;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        AtmosphericChemistry,
        "photolysis_rate",
        &["cross_section", "quantum_yield", "actinic_flux"],
        "Photolysis rate coefficient",
    );
    reg(
        e,
        AtmosphericChemistry,
        "reaction_rate_arrhenius",
        &["prefactor", "activation_energy", "temperature"],
        "Arrhenius reaction rate",
    );
    reg(
        e,
        AtmosphericChemistry,
        "henry_law_concentration",
        &["henry_constant", "partial_pressure"],
        "Henry's law gas solubility",
    );
    reg(
        e,
        AtmosphericChemistry,
        "chemical_lifetime",
        &["rate_constant", "co_reactant_density"],
        "Chemical species lifetime",
    );
    reg(
        e,
        AtmosphericChemistry,
        "mixing_ratio_to_number_density",
        &["mixing_ratio", "pressure", "temperature"],
        "Mixing ratio to number density",
    );
    reg(
        e,
        AtmosphericChemistry,
        "deposition_velocity",
        &[
            "aerodynamic_resistance",
            "surface_resistance",
            "gravitational_settling",
        ],
        "Dry deposition velocity",
    );
    reg(
        e,
        AtmosphericChemistry,
        "aerosol_optical_depth",
        &["extinction_coeff", "layer_thickness"],
        "Aerosol optical depth",
    );
    reg(
        e,
        AtmosphericChemistry,
        "equilibrium_constant_temperature",
        &["k_ref", "delta_h", "t_ref", "temperature"],
        "Van't Hoff equilibrium constant",
    );
    reg(
        e,
        AtmosphericChemistry,
        "lindemann_rate",
        &["k0", "kinf", "m_density"],
        "Lindemann fall-off rate",
    );
    reg(
        e,
        AtmosphericChemistry,
        "mean_free_path",
        &["temperature", "pressure", "collision_diameter"],
        "Molecular mean free path",
    );
    reg(
        e,
        AtmosphericChemistry,
        "column_density",
        &["number_density", "path_length"],
        "Column number density",
    );
}
