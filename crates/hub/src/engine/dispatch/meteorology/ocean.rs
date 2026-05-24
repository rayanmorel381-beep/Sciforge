//! Dispatch handler for ocean coupling functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::meteorology as meteo;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "seawater_density" => Ok(RunOutput::Scalar(meteo::ocean::seawater_density(
            get_f(p, "t")?,
            get_f(p, "s")?,
        ))),
        "sound_speed" => Ok(RunOutput::Scalar(meteo::ocean::sound_speed(
            get_f(p, "t")?,
            get_f(p, "s")?,
            get_f(p, "d")?,
        ))),
        "sea_level_rise_thermal" => Ok(RunOutput::Scalar(meteo::ocean::sea_level_rise_thermal(
            get_f(p, "alpha")?,
            get_f(p, "delta_t")?,
            get_f(p, "d")?,
        ))),
        "henry_solubility" => Ok(RunOutput::Scalar(meteo::ocean::henry_solubility(
            get_f(p, "kh")?,
            get_f(p, "t")?,
            get_f(p, "delta_h")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
