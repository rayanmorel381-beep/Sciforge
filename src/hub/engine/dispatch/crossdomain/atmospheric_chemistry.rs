//! Dispatch handler for atmospheric chemistry functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::cross_domain::atmospheric_chemistry;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

/// Dispatches an atmospheric chemistry function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "photolysis_rate" => Ok(RunOutput::Scalar(atmospheric_chemistry::photolysis_rate(
            get_f(p, "cross_section")?,
            get_f(p, "quantum_yield")?,
            get_f(p, "actinic_flux")?,
        ))),
        "reaction_rate_arrhenius" => Ok(RunOutput::Scalar(
            atmospheric_chemistry::reaction_rate_arrhenius(
                get_f(p, "prefactor")?,
                get_f(p, "activation_energy")?,
                get_f(p, "temperature")?,
            ),
        )),
        "henry_law_concentration" => Ok(RunOutput::Scalar(
            atmospheric_chemistry::henry_law_concentration(
                get_f(p, "henry_constant")?,
                get_f(p, "partial_pressure")?,
            ),
        )),
        "chemical_lifetime" => Ok(RunOutput::Scalar(atmospheric_chemistry::chemical_lifetime(
            get_f(p, "rate_constant")?,
            get_f(p, "co_reactant_density")?,
        ))),
        "mixing_ratio_to_number_density" => Ok(RunOutput::Scalar(
            atmospheric_chemistry::mixing_ratio_to_number_density(
                get_f(p, "mixing_ratio")?,
                get_f(p, "pressure")?,
                get_f(p, "temperature")?,
            ),
        )),
        "deposition_velocity" => Ok(RunOutput::Scalar(
            atmospheric_chemistry::deposition_velocity(
                get_f(p, "aerodynamic_resistance")?,
                get_f(p, "surface_resistance")?,
                get_f(p, "gravitational_settling")?,
            ),
        )),
        "aerosol_optical_depth" => Ok(RunOutput::Scalar(
            atmospheric_chemistry::aerosol_optical_depth(
                get_f(p, "extinction_coeff")?,
                get_f(p, "layer_thickness")?,
            ),
        )),
        "equilibrium_constant_temperature" => Ok(RunOutput::Scalar(
            atmospheric_chemistry::equilibrium_constant_temperature(
                get_f(p, "k_ref")?,
                get_f(p, "delta_h")?,
                get_f(p, "t_ref")?,
                get_f(p, "temperature")?,
            ),
        )),
        "lindemann_rate" => Ok(RunOutput::Scalar(atmospheric_chemistry::lindemann_rate(
            get_f(p, "k0")?,
            get_f(p, "kinf")?,
            get_f(p, "m_density")?,
        ))),
        "mean_free_path" => Ok(RunOutput::Scalar(atmospheric_chemistry::mean_free_path(
            get_f(p, "temperature")?,
            get_f(p, "pressure")?,
            get_f(p, "collision_diameter")?,
        ))),
        "column_density" => Ok(RunOutput::Scalar(atmospheric_chemistry::column_density(
            get_f(p, "number_density")?,
            get_f(p, "path_length")?,
        ))),
        _ => Err(HubError::InvalidInput(format!(
            "atmospheric_chemistry: unknown function: {func}"
        ))),
    }
}
