//! Dispatch handler for radiation transfer functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::meteorology as meteo;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "stefan_boltzmann_flux" => Ok(RunOutput::Scalar(meteo::radiation::stefan_boltzmann_flux(
            get_f(p, "t")?,
        ))),
        "solar_constant" => Ok(RunOutput::Scalar(meteo::radiation::solar_constant())),
        "albedo_reflected" => Ok(RunOutput::Scalar(meteo::radiation::albedo_reflected(
            get_f(p, "solar_flux")?,
            get_f(p, "albedo")?,
        ))),
        "effective_temperature" => Ok(RunOutput::Scalar(meteo::radiation::effective_temperature(
            get_f(p, "solar_flux")?,
            get_f(p, "albedo")?,
        ))),
        "greenhouse_effect" => Ok(RunOutput::Scalar(meteo::radiation::greenhouse_effect(
            get_f(p, "t_surface")?,
            get_f(p, "t_effective")?,
        ))),
        "optical_depth" => Ok(RunOutput::Scalar(meteo::radiation::optical_depth(
            get_f(p, "absorption_coeff")?,
            get_f(p, "path_length")?,
        ))),
        "beer_lambert" => Ok(RunOutput::Scalar(meteo::radiation::beer_lambert(
            get_f(p, "i0")?,
            get_f(p, "tau")?,
        ))),
        "planck_function" => Ok(RunOutput::Scalar(meteo::radiation::planck_function(
            get_f(p, "wavelength")?,
            get_f(p, "t")?,
        ))),
        "solar_zenith_angle" => Ok(RunOutput::Scalar(meteo::radiation::solar_zenith_angle(
            get_f(p, "lat")?,
            get_f(p, "declination")?,
            get_f(p, "hour_angle")?,
        ))),
        "radiative_forcing_co2" => Ok(RunOutput::Scalar(meteo::radiation::radiative_forcing_co2(
            get_f(p, "c")?,
            get_f(p, "c0")?,
        ))),
        "climate_sensitivity" => Ok(RunOutput::Scalar(meteo::radiation::climate_sensitivity(
            get_f(p, "delta_t")?,
            get_f(p, "delta_f")?,
        ))),
        "outgoing_longwave_radiation" => Ok(RunOutput::Scalar(
            meteo::radiation::outgoing_longwave_radiation(get_f(p, "t")?, get_f(p, "emissivity")?),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
