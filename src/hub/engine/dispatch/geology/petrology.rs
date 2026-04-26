//! Dispatch handler for petrology functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::geology as geo;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "cipw_quartz_norm" => Ok(RunOutput::Scalar(geo::petrology::cipw_quartz_norm(
            get_f(p, "sio2")?,
            get_f(p, "feldspars")?,
            get_f(p, "mafics")?,
        ))),
        "mg_number" => Ok(RunOutput::Scalar(geo::petrology::mg_number(
            get_f(p, "mgo")?,
            get_f(p, "feo")?,
        ))),
        "differentiation_index" => Ok(RunOutput::Scalar(geo::petrology::differentiation_index(
            get_f(p, "q")?,
            get_f(p, "or_val")?,
            get_f(p, "ab")?,
            get_f(p, "ne")?,
        ))),
        "total_alkali_silica" => Ok(RunOutput::Scalar(geo::petrology::total_alkali_silica(
            get_f(p, "na2o")?,
            get_f(p, "k2o")?,
        ))),
        "alumina_saturation_index" => {
            Ok(RunOutput::Scalar(geo::petrology::alumina_saturation_index(
                get_f(p, "al2o3")?,
                get_f(p, "cao")?,
                get_f(p, "na2o")?,
                get_f(p, "k2o")?,
            )))
        }
        "color_index" => Ok(RunOutput::Scalar(geo::petrology::color_index(get_v(
            p,
            "mafic_minerals",
        )?))),
        "liquidus_temperature" => Ok(RunOutput::Scalar(geo::petrology::liquidus_temperature(
            get_f(p, "composition")?,
            get_f(p, "t_melt_a")?,
            get_f(p, "t_melt_b")?,
        ))),
        "solidus_depression" => Ok(RunOutput::Scalar(geo::petrology::solidus_depression(
            get_f(p, "water_content")?,
            get_f(p, "base_solidus")?,
            get_f(p, "k")?,
        ))),
        "crystal_settling_velocity" => Ok(RunOutput::Scalar(
            geo::petrology::crystal_settling_velocity(
                get_f(p, "delta_rho")?,
                get_f(p, "g")?,
                get_f(p, "r")?,
                get_f(p, "mu")?,
            ),
        )),
        "viscosity_arrhenius" => Ok(RunOutput::Scalar(geo::petrology::viscosity_arrhenius(
            get_f(p, "a")?,
            get_f(p, "ea")?,
            get_f(p, "t")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
