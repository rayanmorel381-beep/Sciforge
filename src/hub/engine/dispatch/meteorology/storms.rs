//! Dispatch handler for storm dynamics functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::meteorology as meteo;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "potential_intensity" => Ok(RunOutput::Scalar(meteo::storms::potential_intensity(
            get_f(p, "ck")?,
            get_f(p, "cd")?,
            get_f(p, "eta")?,
            get_f(p, "delta_k")?,
        ))),
        "cape" => Ok(RunOutput::Scalar(meteo::storms::cape(
            get_f(p, "t_parcel")?,
            get_f(p, "t_env")?,
            get_f(p, "g")?,
            get_f(p, "dz")?,
        ))),
        "fujita_scale" => Ok(RunOutput::Scalar(
            meteo::storms::fujita_scale(get_f(p, "v")?) as f64,
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
