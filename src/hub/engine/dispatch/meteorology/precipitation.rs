//! Dispatch handler for precipitation functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::meteorology as meteo;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "rain_rate_marshall_palmer" => Ok(RunOutput::Scalar(
            meteo::precipitation::rain_rate_marshall_palmer(get_f(p, "z")?),
        )),
        "radar_reflectivity" => Ok(RunOutput::Scalar(meteo::precipitation::radar_reflectivity(
            get_f(p, "rain_rate")?,
        ))),
        "terminal_velocity_raindrop" => Ok(RunOutput::Scalar(
            meteo::precipitation::terminal_velocity_raindrop(get_f(p, "diameter_mm")?),
        )),
        "thornthwaite_pet" => Ok(RunOutput::Scalar(meteo::precipitation::thornthwaite_pet(
            get_f(p, "t_mean")?,
            get_f(p, "heat_index")?,
            get_f(p, "day_length_hours")?,
        ))),
        "penman_evaporation" => Ok(RunOutput::Scalar(meteo::precipitation::penman_evaporation(
            get_f(p, "delta")?,
            get_f(p, "rn")?,
            get_f(p, "gamma")?,
            get_f(p, "ea")?,
            get_f(p, "u")?,
        ))),
        "intensity_duration_frequency" => Ok(RunOutput::Scalar(
            meteo::precipitation::intensity_duration_frequency(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "duration")?,
                get_f(p, "return_period")?,
            ),
        )),
        "scs_curve_number_runoff" => Ok(RunOutput::Scalar(
            meteo::precipitation::scs_curve_number_runoff(get_f(p, "p")?, get_f(p, "cn")?),
        )),
        "rational_method_runoff" => Ok(RunOutput::Scalar(
            meteo::precipitation::rational_method_runoff(
                get_f(p, "c")?,
                get_f(p, "i")?,
                get_f(p, "a")?,
            ),
        )),
        "unit_hydrograph_peak" => Ok(RunOutput::Scalar(
            meteo::precipitation::unit_hydrograph_peak(get_f(p, "a")?, get_f(p, "tp")?),
        )),
        "antecedent_precipitation_index" => Ok(RunOutput::Scalar(
            meteo::precipitation::antecedent_precipitation_index(
                get_f(p, "prev_api")?,
                get_f(p, "rainfall")?,
                get_f(p, "k")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
