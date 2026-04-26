//! Dispatch handler for atmospheric physics functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::cross_domain::atmospheric_physics;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

/// Dispatches an atmospheric physics function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "planck_radiance" => Ok(RunOutput::Scalar(atmospheric_physics::planck_radiance(
            get_f(p, "wavelength")?,
            get_f(p, "temperature")?,
        ))),
        "stefan_boltzmann_flux" => Ok(RunOutput::Scalar(
            atmospheric_physics::stefan_boltzmann_flux(get_f(p, "temperature")?),
        )),
        "brightness_temperature" => Ok(RunOutput::Scalar(
            atmospheric_physics::brightness_temperature(
                get_f(p, "radiance")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "rayleigh_scattering_cross_section" => Ok(RunOutput::Scalar(
            atmospheric_physics::rayleigh_scattering_cross_section(
                get_f(p, "wavelength")?,
                get_f(p, "refractive_index")?,
                get_f(p, "number_density")?,
            ),
        )),
        "optical_depth" => Ok(RunOutput::Scalar(atmospheric_physics::optical_depth(
            get_f(p, "cross_section")?,
            get_f(p, "number_density")?,
            get_f(p, "path_length")?,
        ))),
        "atmospheric_scale_height" => Ok(RunOutput::Scalar(
            atmospheric_physics::atmospheric_scale_height(
                get_f(p, "temperature")?,
                get_f(p, "mean_molecular_mass")?,
                get_f(p, "gravity")?,
            ),
        )),
        "pressure_at_altitude" => Ok(RunOutput::Scalar(
            atmospheric_physics::pressure_at_altitude(
                get_f(p, "surface_pressure")?,
                get_f(p, "scale_height")?,
                get_f(p, "altitude")?,
            ),
        )),
        "dry_adiabatic_lapse_rate" => Ok(RunOutput::Scalar(
            atmospheric_physics::dry_adiabatic_lapse_rate(
                get_f(p, "gravity")?,
                get_f(p, "specific_heat")?,
            ),
        )),
        "wien_peak_wavelength" => Ok(RunOutput::Scalar(
            atmospheric_physics::wien_peak_wavelength(get_f(p, "temperature")?),
        )),
        "schwarzschild_radiative_transfer" => Ok(RunOutput::Scalar(
            atmospheric_physics::schwarzschild_radiative_transfer(
                get_f(p, "source_function")?,
                get_f(p, "initial_radiance")?,
                get_f(p, "optical_depth")?,
            ),
        )),
        "effective_emission_temperature" => Ok(RunOutput::Scalar(
            atmospheric_physics::effective_emission_temperature(
                get_f(p, "outgoing_flux")?,
                get_f(p, "emissivity")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!(
            "atmospheric_physics: unknown function: {func}"
        ))),
    }
}
