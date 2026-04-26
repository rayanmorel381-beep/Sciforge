//! Dispatch handler for galactic dynamics functions.

use super::super::params::*;
use crate::hub::domain::astronomy as astro;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "galactic_rotation_velocity" => Ok(RunOutput::Scalar(
            astro::galactic::galactic_rotation_velocity(get_f(p, "r")?),
        )),
        "sun_orbital_period" => Ok(RunOutput::Scalar(astro::galactic::sun_orbital_period())),
        "sun_orbital_velocity" => Ok(RunOutput::Scalar(astro::galactic::sun_orbital_velocity())),
        "sun_galactocentric_distance" => Ok(RunOutput::Scalar(
            astro::galactic::sun_galactocentric_distance(),
        )),
        "galactic_mass_within_radius" => Ok(RunOutput::Scalar(
            astro::galactic::galactic_mass_within_radius(get_f(p, "r")?),
        )),
        "sgr_a_schwarzschild_radius" => Ok(RunOutput::Scalar(
            astro::galactic::sgr_a_schwarzschild_radius(),
        )),
        "sgr_a_sphere_of_influence" => Ok(RunOutput::Scalar(
            astro::galactic::sgr_a_sphere_of_influence(),
        )),
        "sgr_a_distance" => Ok(RunOutput::Scalar(astro::galactic::sgr_a_distance())),
        "m31_approach_time" => Ok(RunOutput::Scalar(astro::galactic::m31_approach_time())),
        "m31_reduced_mass" => Ok(RunOutput::Scalar(astro::galactic::m31_reduced_mass())),
        "hubble_recession_velocity" => Ok(RunOutput::Scalar(
            astro::galactic::hubble_recession_velocity(get_f(p, "distance_mpc")?),
        )),
        "galactic_disk_density" => Ok(RunOutput::Scalar(astro::galactic::galactic_disk_density(
            get_f(p, "r")?,
            get_f(p, "z")?,
        ))),
        "galactic_escape_velocity" => Ok(RunOutput::Scalar(
            astro::galactic::galactic_escape_velocity(get_f(p, "r")?),
        )),
        "galactic_circular_velocity" => Ok(RunOutput::Scalar(
            astro::galactic::galactic_circular_velocity(get_f(p, "r")?),
        )),
        "galactic_dynamical_time" => Ok(RunOutput::Scalar(
            astro::galactic::galactic_dynamical_time(get_f(p, "r")?),
        )),
        "stellar_number_density" => Ok(RunOutput::Scalar(astro::galactic::stellar_number_density(
            get_f(p, "r")?,
            get_f(p, "z")?,
        ))),
        "oort_a_constant" => Ok(RunOutput::Scalar(astro::galactic::oort_a_constant(
            get_f(p, "r")?,
            get_f(p, "dr")?,
        ))),
        "oort_b_constant" => Ok(RunOutput::Scalar(astro::galactic::oort_b_constant(
            get_f(p, "r")?,
            get_f(p, "dr")?,
        ))),
        "epicyclic_frequency" => Ok(RunOutput::Scalar(astro::galactic::epicyclic_frequency(
            get_f(p, "r")?,
            get_f(p, "dr")?,
        ))),
        "bulge_mass_within" => Ok(RunOutput::Scalar(astro::galactic::bulge_mass_within(
            get_f(p, "r")?,
        ))),
        "tidal_radius" => Ok(RunOutput::Scalar(astro::galactic::tidal_radius(
            get_f(p, "m_cluster")?,
            get_f(p, "r_galactic")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
