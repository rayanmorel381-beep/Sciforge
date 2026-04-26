//! Dispatch handler for rotational dynamics functions.

use super::super::params::*;
use crate::hub::domain::astronomy as astro;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "surface_velocity_at_latitude" => Ok(RunOutput::Scalar(
            astro::rotation::surface_velocity_at_latitude(
                get_f(p, "omega")?,
                get_f(p, "r")?,
                get_f(p, "phi")?,
            ),
        )),
        "centripetal_acceleration" => Ok(RunOutput::Scalar(
            astro::rotation::centripetal_acceleration(
                get_f(p, "omega")?,
                get_f(p, "r")?,
                get_f(p, "phi")?,
            ),
        )),
        "coriolis_parameter" => Ok(RunOutput::Scalar(astro::rotation::coriolis_parameter(
            get_f(p, "omega")?,
            get_f(p, "phi")?,
        ))),
        "moment_of_inertia" => Ok(RunOutput::Scalar(astro::rotation::moment_of_inertia(
            get_f(p, "c_factor")?,
            get_f(p, "m")?,
            get_f(p, "r")?,
        ))),
        "rotational_kinetic_energy" => Ok(RunOutput::Scalar(
            astro::rotation::rotational_kinetic_energy(get_f(p, "inertia")?, get_f(p, "omega")?),
        )),
        "nutation_obliquity_rad" => Ok(RunOutput::Scalar(astro::rotation::nutation_obliquity_rad(
            get_f(p, "omega_node")?,
        ))),
        "day_length_variation" => Ok(RunOutput::Scalar(astro::rotation::day_length_variation(
            get_f(p, "doy")?,
            get_f(p, "latitude")?,
            get_f(p, "tilt")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
