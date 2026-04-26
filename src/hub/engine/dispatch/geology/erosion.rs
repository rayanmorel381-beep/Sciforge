//! Dispatch handler for erosion functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::geology as geo;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "fluvial_erosion_rate" => Ok(RunOutput::Scalar(geo::erosion::fluvial_erosion_rate(
            get_f(p, "k")?,
            get_f(p, "p")?,
            get_f(p, "alpha")?,
            get_f(p, "vc")?,
        ))),
        "chemical_weathering_rate" => {
            Ok(RunOutput::Scalar(geo::erosion::chemical_weathering_rate(
                get_f(p, "a")?,
                get_f(p, "ea")?,
                get_f(p, "t")?,
                get_f(p, "p")?,
            )))
        }
        "frost_weathering_rate" => Ok(RunOutput::Scalar(geo::erosion::frost_weathering_rate(
            get_f(p, "n_ft")?,
            get_f(p, "phi")?,
        ))),
        "wind_erosion_threshold" => Ok(RunOutput::Scalar(geo::erosion::wind_erosion_threshold(
            get_f(p, "rho_p")?,
            get_f(p, "rho_a")?,
            get_f(p, "g")?,
            get_f(p, "d")?,
        ))),
        "volcanic_explosivity_index" => Ok(RunOutput::Scalar(
            geo::erosion::volcanic_explosivity_index(get_f(p, "volume_km3")?) as f64,
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
