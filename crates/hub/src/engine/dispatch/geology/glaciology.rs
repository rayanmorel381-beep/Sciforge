//! Dispatch handler for glaciology functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::geology as geo;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "glen_strain_rate" => Ok(RunOutput::Scalar(geo::glaciology::glen_strain_rate(
            get_f(p, "a")?,
            get_f(p, "tau")?,
            get_f(p, "n")?,
        ))),
        "shallow_ice_velocity" => Ok(RunOutput::Scalar(geo::glaciology::shallow_ice_velocity(
            get_f(p, "a")?,
            get_f(p, "n")?,
            get_f(p, "rho")?,
            get_f(p, "g")?,
            get_f(p, "alpha")?,
            get_f(p, "h")?,
        ))),
        "ice_viscosity" => Ok(RunOutput::Scalar(geo::glaciology::ice_viscosity(
            get_f(p, "a")?,
            get_f(p, "tau")?,
            get_f(p, "n")?,
        ))),
        "glacial_bed_erosion" => Ok(RunOutput::Scalar(geo::glaciology::glacial_bed_erosion(
            get_f(p, "kg")?,
            get_f(p, "vb")?,
            get_f(p, "l")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
