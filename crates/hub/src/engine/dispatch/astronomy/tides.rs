//! Dispatch handler for tidal dynamics functions.

use super::super::params::*;
use crate::domain::astronomy as astro;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "tidal_potential" => Ok(RunOutput::Scalar(astro::tides::tidal_potential(
            get_f(p, "m")?,
            get_f(p, "r")?,
            get_f(p, "d")?,
            get_f(p, "k2")?,
            get_f(p, "theta")?,
        ))),
        "tidal_bulge_height" => Ok(RunOutput::Scalar(astro::tides::tidal_bulge_height(
            get_f(p, "a_tidal")?,
            get_f(p, "r")?,
            get_f(p, "g")?,
            get_f(p, "h2")?,
        ))),
        "spring_tide_amplitude" => Ok(RunOutput::Scalar(astro::tides::spring_tide_amplitude(
            get_f(p, "h_moon")?,
            get_f(p, "h_sun")?,
        ))),
        "neap_tide_amplitude" => Ok(RunOutput::Scalar(astro::tides::neap_tide_amplitude(
            get_f(p, "h_moon")?,
            get_f(p, "h_sun")?,
        ))),
        "tidal_dissipation_rate" => Ok(RunOutput::Scalar(astro::tides::tidal_dissipation_rate(
            get_f(p, "k2")?,
            get_f(p, "n")?,
            get_f(p, "m")?,
            get_f(p, "r")?,
            get_f(p, "q")?,
            get_f(p, "d")?,
        ))),
        "tidal_locking_timescale" => Ok(RunOutput::Scalar(astro::tides::tidal_locking_timescale(
            get_f(p, "omega")?,
            get_f(p, "a")?,
            get_f(p, "mu")?,
            get_f(p, "q")?,
            get_f(p, "m")?,
            get_f(p, "r")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
