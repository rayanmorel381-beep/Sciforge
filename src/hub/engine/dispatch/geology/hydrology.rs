//! Dispatch handler for hydrology functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::geology as geo;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "manning_velocity" => Ok(RunOutput::Scalar(geo::hydrology::manning_velocity(
            get_f(p, "n")?,
            get_f(p, "rh")?,
            get_f(p, "s")?,
        ))),
        "chezy_velocity" => Ok(RunOutput::Scalar(geo::hydrology::chezy_velocity(
            get_f(p, "c")?,
            get_f(p, "rh")?,
            get_f(p, "s")?,
        ))),
        "froude_number" => Ok(RunOutput::Scalar(geo::hydrology::froude_number(
            get_f(p, "v")?,
            get_f(p, "g")?,
            get_f(p, "d")?,
        ))),
        "reynolds_number" => Ok(RunOutput::Scalar(geo::hydrology::reynolds_number(
            get_f(p, "v")?,
            get_f(p, "d")?,
            get_f(p, "nu")?,
        ))),
        "stream_power" => Ok(RunOutput::Scalar(geo::hydrology::stream_power(
            get_f(p, "rho")?,
            get_f(p, "g")?,
            get_f(p, "q")?,
            get_f(p, "s")?,
        ))),
        "hjulstrom_erosion_threshold" => Ok(RunOutput::Scalar(
            geo::hydrology::hjulstrom_erosion_threshold(get_f(p, "d_grain")?),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
