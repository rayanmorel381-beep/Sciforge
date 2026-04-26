//! Dispatch handler for celestial mechanics functions.

use super::super::params::*;
use crate::hub::domain::astronomy as astro;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "julian_date_to_j2000_century" => Ok(RunOutput::Scalar(
            astro::celestial::julian_date_to_j2000_century(get_f(p, "jd")?),
        )),
        "mean_obliquity_ecliptic" => Ok(RunOutput::Scalar(
            astro::celestial::mean_obliquity_ecliptic(get_f(p, "t_century")?),
        )),
        "nutation_longitude" => Ok(RunOutput::Scalar(astro::celestial::nutation_longitude(
            get_f(p, "omega")?,
        ))),
        "equation_of_time" => Ok(RunOutput::Scalar(astro::celestial::equation_of_time(
            get_f(p, "day_of_year")?,
        ))),
        "sidereal_year_seconds" => Ok(RunOutput::Scalar(astro::celestial::sidereal_year_seconds())),
        "tropical_year_seconds" => Ok(RunOutput::Scalar(astro::celestial::tropical_year_seconds())),
        "precession_period" => Ok(RunOutput::Scalar(astro::celestial::precession_period())),
        "tidal_heating" => Ok(RunOutput::Scalar(astro::celestial::tidal_heating(
            get_f(p, "mass_primary")?,
            get_f(p, "radius")?,
            get_f(p, "eccentricity")?,
            get_f(p, "a")?,
            get_f(p, "n")?,
        ))),
        "gravitational_force" => Ok(RunOutput::Scalar(astro::celestial::gravitational_force(
            get_f(p, "m1")?,
            get_f(p, "m2")?,
            get_f(p, "r")?,
        ))),
        "tidal_force" => Ok(RunOutput::Scalar(astro::celestial::tidal_force(
            get_f(p, "m")?,
            get_f(p, "r")?,
            get_f(p, "delta_r")?,
        ))),
        "synodic_period" => Ok(RunOutput::Scalar(astro::celestial::synodic_period(
            get_f(p, "p1")?,
            get_f(p, "p2")?,
        ))),
        "apparent_angular_size" => Ok(RunOutput::Scalar(astro::celestial::apparent_angular_size(
            get_f(p, "diameter")?,
            get_f(p, "distance")?,
        ))),
        "parallax_distance" => Ok(RunOutput::Scalar(astro::celestial::parallax_distance(
            get_f(p, "parallax_arcsec")?,
        ))),
        "barycenter" => Ok(RunOutput::Scalar(astro::celestial::barycenter(
            get_f(p, "m1")?,
            get_f(p, "m2")?,
            get_f(p, "d")?,
        ))),
        "lagrange_l1" => Ok(RunOutput::Scalar(astro::celestial::lagrange_l1(
            get_f(p, "d")?,
            get_f(p, "m1")?,
            get_f(p, "m2")?,
        ))),
        "hill_sphere" => Ok(RunOutput::Scalar(astro::celestial::hill_sphere(
            get_f(p, "a")?,
            get_f(p, "m")?,
            get_f(p, "m_star")?,
            get_f(p, "e")?,
        ))),
        "surface_gravity" => Ok(RunOutput::Scalar(astro::celestial::surface_gravity(
            get_f(p, "m")?,
            get_f(p, "r")?,
        ))),
        "sidereal_to_solar" => Ok(RunOutput::Scalar(astro::celestial::sidereal_to_solar(
            get_f(p, "sidereal_period")?,
            get_f(p, "orbital_period")?,
        ))),
        "habitable_zone_inner" => Ok(RunOutput::Scalar(astro::celestial::habitable_zone_inner(
            get_f(p, "luminosity_solar")?,
        ))),
        "habitable_zone_outer" => Ok(RunOutput::Scalar(astro::celestial::habitable_zone_outer(
            get_f(p, "luminosity_solar")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
