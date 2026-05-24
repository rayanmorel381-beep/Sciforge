//! Dispatch handler for atmospheric dynamics functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::meteorology as meteo;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "coriolis_parameter" => Ok(RunOutput::Scalar(meteo::dynamics::coriolis_parameter(
            get_f(p, "latitude")?,
            get_f(p, "omega")?,
        ))),
        "geostrophic_wind" => {
            let r = meteo::dynamics::geostrophic_wind(
                get_f(p, "dp_dx")?,
                get_f(p, "dp_dy")?,
                get_f(p, "rho")?,
                get_f(p, "f")?,
            );
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "rossby_number" => Ok(RunOutput::Scalar(meteo::dynamics::rossby_number(
            get_f(p, "u")?,
            get_f(p, "l")?,
            get_f(p, "f")?,
        ))),
        "rossby_wave_speed" => Ok(RunOutput::Scalar(meteo::dynamics::rossby_wave_speed(
            get_f(p, "beta")?,
            get_f(p, "k")?,
        ))),
        "thermal_wind" => Ok(RunOutput::Scalar(meteo::dynamics::thermal_wind(
            get_f(p, "f")?,
            get_f(p, "delta_t")?,
            get_f(p, "delta_x")?,
            get_f(p, "t_mean")?,
        ))),
        "potential_vorticity" => Ok(RunOutput::Scalar(meteo::dynamics::potential_vorticity(
            get_f(p, "f")?,
            get_f(p, "dtheta_dp")?,
        ))),
        "ekman_depth" => Ok(RunOutput::Scalar(meteo::dynamics::ekman_depth(
            get_f(p, "nu")?,
            get_f(p, "f")?,
        ))),
        "richardson_number" => Ok(RunOutput::Scalar(meteo::dynamics::richardson_number(
            get_f(p, "n2")?,
            get_f(p, "du_dz")?,
        ))),
        "rossby_deformation_radius" => Ok(RunOutput::Scalar(
            meteo::dynamics::rossby_deformation_radius(
                get_f(p, "n")?,
                get_f(p, "h")?,
                get_f(p, "f")?,
            ),
        )),
        "cyclone_gradient_wind" => Ok(RunOutput::Scalar(meteo::dynamics::cyclone_gradient_wind(
            get_f(p, "r")?,
            get_f(p, "f")?,
            get_f(p, "dp_dr")?,
            get_f(p, "rho")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
