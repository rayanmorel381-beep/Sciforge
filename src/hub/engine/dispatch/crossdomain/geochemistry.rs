//! Dispatch handler for geochemistry functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::cross_domain::geochemistry;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

/// Dispatches a geochemistry function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "partition_coefficient" => Ok(RunOutput::Scalar(geochemistry::partition_coefficient(
            get_f(p, "c_solid")?,
            get_f(p, "c_liquid")?,
        ))),
        "batch_melting" => Ok(RunOutput::Scalar(geochemistry::batch_melting(
            get_f(p, "c0")?,
            get_f(p, "melt_fraction")?,
            get_f(p, "bulk_d")?,
        ))),
        "fractional_crystallization" => {
            Ok(RunOutput::Scalar(geochemistry::fractional_crystallization(
                get_f(p, "c0")?,
                get_f(p, "fraction_remaining")?,
                get_f(p, "bulk_d")?,
            )))
        }
        "delta_notation" => Ok(RunOutput::Scalar(geochemistry::delta_notation(
            get_f(p, "r_sample")?,
            get_f(p, "r_standard")?,
        ))),
        "rayleigh_fractionation" => Ok(RunOutput::Scalar(geochemistry::rayleigh_fractionation(
            get_f(p, "r0")?,
            get_f(p, "fraction_remaining")?,
            get_f(p, "alpha")?,
        ))),
        "weathering_rate" => Ok(RunOutput::Scalar(geochemistry::weathering_rate(
            get_f(p, "rate_constant")?,
            get_f(p, "surface_area")?,
            get_f(p, "saturation_state")?,
        ))),
        "activity_coefficient_debye_huckel" => Ok(RunOutput::Scalar(
            geochemistry::activity_coefficient_debye_huckel(
                get_f(p, "z")?,
                get_f(p, "ionic_strength")?,
            ),
        )),
        "solubility_product_temperature" => Ok(RunOutput::Scalar(
            geochemistry::solubility_product_temperature(
                get_f(p, "ksp0")?,
                get_f(p, "delta_h")?,
                get_f(p, "t")?,
                get_f(p, "t0")?,
            ),
        )),
        "eh_ph_boundary" => Ok(RunOutput::Scalar(geochemistry::eh_ph_boundary(
            get_f(p, "e0")?,
            get_f(p, "n_electrons")?,
            get_f(p, "n_protons")?,
            get_f(p, "ph")?,
            get_f(p, "temperature")?,
        ))),
        "distribution_coefficient" => {
            Ok(RunOutput::Scalar(geochemistry::distribution_coefficient(
                get_f(p, "c_adsorbed")?,
                get_f(p, "c_solution")?,
            )))
        }
        "mixing_two_component" => Ok(RunOutput::Scalar(geochemistry::mixing_two_component(
            get_f(p, "c1")?,
            get_f(p, "c2")?,
            get_f(p, "fraction_1")?,
        ))),
        _ => Err(HubError::InvalidInput(format!(
            "geochemistry: unknown function: {func}"
        ))),
    }
}
