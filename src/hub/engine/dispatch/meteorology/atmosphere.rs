//! Dispatch handler for atmospheric thermodynamics functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::meteorology as meteo;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "barometric_formula" => Ok(RunOutput::Scalar(meteo::atmosphere::barometric_formula(
            get_f(p, "p0")?,
            get_f(p, "m")?,
            get_f(p, "g")?,
            get_f(p, "h")?,
            get_f(p, "t")?,
        ))),
        "scale_height" => Ok(RunOutput::Scalar(meteo::atmosphere::scale_height(
            get_f(p, "t")?,
            get_f(p, "m")?,
            get_f(p, "g")?,
        ))),
        "lapse_rate_dry" => Ok(RunOutput::Scalar(meteo::atmosphere::lapse_rate_dry())),
        "lapse_rate_moist" => Ok(RunOutput::Scalar(meteo::atmosphere::lapse_rate_moist(
            get_f(p, "t")?,
            get_f(p, "l_v")?,
            get_f(p, "r_s")?,
        ))),
        "potential_temperature" => Ok(RunOutput::Scalar(meteo::atmosphere::potential_temperature(
            get_f(p, "t")?,
            get_f(p, "p")?,
            get_f(p, "p0")?,
        ))),
        "virtual_temperature" => Ok(RunOutput::Scalar(meteo::atmosphere::virtual_temperature(
            get_f(p, "t")?,
            get_f(p, "r")?,
        ))),
        "mixing_ratio" => Ok(RunOutput::Scalar(meteo::atmosphere::mixing_ratio(
            get_f(p, "e")?,
            get_f(p, "p")?,
        ))),
        "saturation_vapor_pressure" => Ok(RunOutput::Scalar(
            meteo::atmosphere::saturation_vapor_pressure(get_f(p, "t_celsius")?),
        )),
        "relative_humidity" => Ok(RunOutput::Scalar(meteo::atmosphere::relative_humidity(
            get_f(p, "e")?,
            get_f(p, "es")?,
        ))),
        "dew_point" => Ok(RunOutput::Scalar(meteo::atmosphere::dew_point(
            get_f(p, "t")?,
            get_f(p, "rh")?,
        ))),
        "density_altitude" => Ok(RunOutput::Scalar(meteo::atmosphere::density_altitude(
            get_f(p, "pressure_altitude")?,
            get_f(p, "temperature_c")?,
        ))),
        "brunt_vaisala_frequency" => Ok(RunOutput::Scalar(
            meteo::atmosphere::brunt_vaisala_frequency(
                get_f(p, "g")?,
                get_f(p, "theta")?,
                get_f(p, "dtheta_dz")?,
            ),
        )),
        "mie_phase" => Ok(RunOutput::Scalar(meteo::atmosphere::mie_phase(
            get_f(p, "cos_theta")?,
            get_f(p, "g")?,
        ))),
        "rayleigh_phase" => Ok(RunOutput::Scalar(meteo::atmosphere::rayleigh_phase(get_f(
            p,
            "cos_theta",
        )?))),
        "lifted_condensation_level" => Ok(RunOutput::Scalar(
            meteo::atmosphere::lifted_condensation_level(
                get_f(p, "t_surface")?,
                get_f(p, "dew_point")?,
            ),
        )),
        "dry_adiabatic_temperature" => Ok(RunOutput::Scalar(
            meteo::atmosphere::dry_adiabatic_temperature(
                get_f(p, "t_surface")?,
                get_f(p, "altitude")?,
            ),
        )),
        "convective_available_potential_energy" => Ok(RunOutput::Scalar(
            meteo::atmosphere::convective_available_potential_energy(
                get_v(p, "t_parcel")?,
                get_v(p, "t_env")?,
                get_f(p, "dz")?,
            ),
        )),
        "convective_inhibition" => Ok(RunOutput::Scalar(meteo::atmosphere::convective_inhibition(
            get_v(p, "t_parcel")?,
            get_v(p, "t_env")?,
            get_f(p, "dz")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
