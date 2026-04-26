//! Dispatch handler for wind dynamics functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::meteorology as meteo;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "beaufort_to_m_s" => Ok(RunOutput::Scalar(meteo::winds::beaufort_to_m_s(get_f(
            p, "b",
        )?))),
        "wind_chill" => Ok(RunOutput::Scalar(meteo::winds::wind_chill(
            get_f(p, "t")?,
            get_f(p, "v")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
