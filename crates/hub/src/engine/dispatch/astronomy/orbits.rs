//! Dispatch handler for orbital mechanics functions.

use super::super::params::*;
use crate::domain::astronomy as astro;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "kepler_period" => Ok(RunOutput::Scalar(astro::orbits::kepler_period(
            get_f(p, "a")?,
            get_f(p, "mu")?,
        ))),
        "kepler_velocity" => Ok(RunOutput::Scalar(astro::orbits::kepler_velocity(
            get_f(p, "mu")?,
            get_f(p, "r")?,
            get_f(p, "a")?,
        ))),
        "circular_velocity" => Ok(RunOutput::Scalar(astro::orbits::circular_velocity(
            get_f(p, "mu")?,
            get_f(p, "r")?,
        ))),
        "escape_velocity" => Ok(RunOutput::Scalar(astro::orbits::escape_velocity(
            get_f(p, "mu")?,
            get_f(p, "r")?,
        ))),
        "vis_viva" => Ok(RunOutput::Scalar(astro::orbits::vis_viva(
            get_f(p, "mu")?,
            get_f(p, "r")?,
            get_f(p, "a")?,
        ))),
        "orbital_energy" => Ok(RunOutput::Scalar(astro::orbits::orbital_energy(
            get_f(p, "mu")?,
            get_f(p, "a")?,
        ))),
        "angular_momentum" => Ok(RunOutput::Scalar(astro::orbits::angular_momentum(
            get_f(p, "mu")?,
            get_f(p, "a")?,
            get_f(p, "e")?,
        ))),
        "periapsis" => Ok(RunOutput::Scalar(astro::orbits::periapsis(
            get_f(p, "a")?,
            get_f(p, "e")?,
        ))),
        "apoapsis" => Ok(RunOutput::Scalar(astro::orbits::apoapsis(
            get_f(p, "a")?,
            get_f(p, "e")?,
        ))),
        "true_anomaly_to_radius" => Ok(RunOutput::Scalar(astro::orbits::true_anomaly_to_radius(
            get_f(p, "a")?,
            get_f(p, "e")?,
            get_f(p, "theta")?,
        ))),
        "hohmann_delta_v" => Ok(RunOutput::Scalar(astro::orbits::hohmann_delta_v(
            get_f(p, "mu")?,
            get_f(p, "r1")?,
            get_f(p, "r2")?,
        ))),
        "sphere_of_influence" => Ok(RunOutput::Scalar(astro::orbits::sphere_of_influence(
            get_f(p, "a")?,
            get_f(p, "m_planet")?,
            get_f(p, "m_star")?,
        ))),
        "roche_limit" => Ok(RunOutput::Scalar(astro::orbits::roche_limit(
            get_f(p, "r_primary")?,
            get_f(p, "rho_primary")?,
            get_f(p, "rho_secondary")?,
        ))),
        "solve_kepler" => Ok(RunOutput::Scalar(astro::orbits::solve_kepler(
            get_f(p, "m")?,
            get_f(p, "e")?,
            get_f(p, "tol")?,
        ))),
        "gr_perihelion_precession" => {
            Ok(RunOutput::Scalar(astro::orbits::gr_perihelion_precession(
                get_f(p, "mass")?,
                get_f(p, "a")?,
                get_f(p, "e")?,
            )))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
