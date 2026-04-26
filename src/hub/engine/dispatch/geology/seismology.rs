//! Dispatch handler for seismology functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::geology as geo;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "p_wave_velocity" => Ok(RunOutput::Scalar(geo::seismology::p_wave_velocity(
            get_f(p, "k")?,
            get_f(p, "g")?,
            get_f(p, "rho")?,
        ))),
        "s_wave_velocity" => Ok(RunOutput::Scalar(geo::seismology::s_wave_velocity(
            get_f(p, "g")?,
            get_f(p, "rho")?,
        ))),
        "richter_magnitude" => Ok(RunOutput::Scalar(geo::seismology::richter_magnitude(
            get_f(p, "amplitude")?,
            get_f(p, "distance_km")?,
        ))),
        "moment_magnitude" => Ok(RunOutput::Scalar(geo::seismology::moment_magnitude(get_f(
            p,
            "seismic_moment",
        )?))),
        "seismic_moment" => Ok(RunOutput::Scalar(geo::seismology::seismic_moment(get_f(
            p, "mw",
        )?))),
        "epicenter_distance" => Ok(RunOutput::Scalar(geo::seismology::epicenter_distance(
            get_f(p, "vp")?,
            get_f(p, "vs")?,
            get_f(p, "ts_tp")?,
        ))),
        "travel_time" => Ok(RunOutput::Scalar(geo::seismology::travel_time(
            get_f(p, "distance")?,
            get_f(p, "velocity")?,
        ))),
        "snell_seismic" => Ok(RunOutput::Scalar(geo::seismology::snell_seismic(
            get_f(p, "v1")?,
            get_f(p, "theta1")?,
            get_f(p, "v2")?,
        ))),
        "gutenberg_richter" => Ok(RunOutput::Scalar(geo::seismology::gutenberg_richter(
            get_f(p, "a")?,
            get_f(p, "b")?,
            get_f(p, "magnitude")?,
        ))),
        "omori_aftershock" => Ok(RunOutput::Scalar(geo::seismology::omori_aftershock(
            get_f(p, "k")?,
            get_f(p, "c")?,
            get_f(p, "p")?,
            get_f(p, "t")?,
        ))),
        "seismic_energy" => Ok(RunOutput::Scalar(geo::seismology::seismic_energy(get_f(
            p,
            "magnitude",
        )?))),
        "peak_ground_acceleration" => Ok(RunOutput::Scalar(
            geo::seismology::peak_ground_acceleration(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "magnitude")?,
                get_f(p, "distance")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
