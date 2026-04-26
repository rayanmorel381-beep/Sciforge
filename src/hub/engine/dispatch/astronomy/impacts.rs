//! Dispatch handler for impact mechanics functions.

use super::super::params::*;
use crate::hub::domain::astronomy as astro;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "crater_diameter" => Ok(RunOutput::Scalar(astro::impacts::crater_diameter(
            get_f(p, "rho_i")?,
            get_f(p, "d_p")?,
            get_f(p, "v")?,
            get_f(p, "g")?,
            get_f(p, "rho_t")?,
        ))),
        "fireball_radius" => Ok(RunOutput::Scalar(astro::impacts::fireball_radius(get_f(
            p, "e_kt",
        )?))),
        "ejecta_volume" => Ok(RunOutput::Scalar(astro::impacts::ejecta_volume(
            get_f(p, "d")?,
            get_f(p, "depth")?,
        ))),
        "impact_velocity" => Ok(RunOutput::Scalar(astro::impacts::impact_velocity(
            get_f(p, "v_inf")?,
            get_f(p, "v_esc")?,
        ))),
        "ejecta_escape_fraction" => Ok(RunOutput::Scalar(astro::impacts::ejecta_escape_fraction(
            get_f(p, "v_esc")?,
            get_f(p, "v_ejecta")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
