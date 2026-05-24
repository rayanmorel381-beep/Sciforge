//! Dispatch handler for astrobiology functions.

use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::cross_domain::astrobiology;
use crate::engine::dispatch::params::*;
use crate::engine::experience::runner::RunOutput;

/// Dispatches an astrobiology function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "planet_equilibrium_temperature" => Ok(RunOutput::Scalar(
            astrobiology::planet_equilibrium_temperature(
                get_f(p, "stellar_luminosity")?,
                get_f(p, "distance")?,
                get_f(p, "albedo")?,
            ),
        )),
        "habitable_zone_inner" => Ok(RunOutput::Scalar(astrobiology::habitable_zone_inner(
            get_f(p, "luminosity")?,
        ))),
        "habitable_zone_outer" => Ok(RunOutput::Scalar(astrobiology::habitable_zone_outer(
            get_f(p, "luminosity")?,
        ))),
        "atmospheric_escape_parameter" => Ok(RunOutput::Scalar(
            astrobiology::atmospheric_escape_parameter(
                get_f(p, "temperature")?,
                get_f(p, "planet_mass")?,
                get_f(p, "planet_radius")?,
                get_f(p, "molecular_mass")?,
            ),
        )),
        "tidal_locking_timescale" => Ok(RunOutput::Scalar(astrobiology::tidal_locking_timescale(
            get_f(p, "planet_mass")?,
            get_f(p, "semi_major")?,
            get_f(p, "star_mass")?,
            get_f(p, "planet_radius")?,
            get_f(p, "tidal_quality")?,
        ))),
        "energy_limited_mass_loss" => {
            Ok(RunOutput::Scalar(astrobiology::energy_limited_mass_loss(
                get_f(p, "xuv_flux")?,
                get_f(p, "efficiency")?,
                get_f(p, "planet_mass")?,
                get_f(p, "planet_radius")?,
            )))
        }
        "biosignature_column_density" => Ok(RunOutput::Scalar(
            astrobiology::biosignature_column_density(
                get_f(p, "mixing_ratio")?,
                get_f(p, "surface_pressure")?,
                get_f(p, "gravity")?,
                get_f(p, "mean_molecular_mass")?,
            ),
        )),
        "uv_surface_flux" => Ok(RunOutput::Scalar(astrobiology::uv_surface_flux(
            get_f(p, "incident_flux")?,
            get_f(p, "optical_depth")?,
        ))),
        "photosynthetic_flux_limit" => {
            Ok(RunOutput::Scalar(astrobiology::photosynthetic_flux_limit(
                get_f(p, "photon_flux")?,
                get_f(p, "quantum_efficiency")?,
                get_f(p, "photon_energy")?,
            )))
        }
        "drake_equation" => Ok(RunOutput::Scalar(astrobiology::drake_equation(
            get_f(p, "rate_star_formation")?,
            get_f(p, "fraction_planets")?,
            get_f(p, "habitable_per_system")?,
            get_f(p, "fraction_life")?,
            get_f(p, "fraction_intelligence")?,
            get_f(p, "fraction_communication")?,
            get_f(p, "civilization_lifetime")?,
        ))),
        "surface_gravity" => Ok(RunOutput::Scalar(astrobiology::surface_gravity(
            get_f(p, "mass")?,
            get_f(p, "radius")?,
        ))),
        _ => Err(HubError::InvalidInput(format!(
            "astrobiology: unknown function: {func}"
        ))),
    }
}
