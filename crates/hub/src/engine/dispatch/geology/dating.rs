//! Dispatch handler for radiometric dating functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::geology as geo;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "radioactive_decay" => Ok(RunOutput::Scalar(geo::dating::radioactive_decay(
            get_f(p, "n0")?,
            get_f(p, "lambda")?,
            get_f(p, "t")?,
        ))),
        "half_life" => Ok(RunOutput::Scalar(geo::dating::half_life(get_f(
            p, "lambda",
        )?))),
        "decay_constant" => Ok(RunOutput::Scalar(geo::dating::decay_constant(get_f(
            p,
            "half_life",
        )?))),
        "age_from_ratio" => Ok(RunOutput::Scalar(geo::dating::age_from_ratio(
            get_f(p, "ratio_daughter_parent")?,
            get_f(p, "lambda")?,
        ))),
        "carbon14_age" => Ok(RunOutput::Scalar(geo::dating::carbon14_age(get_f(
            p, "ratio",
        )?))),
        "potassium_argon_age" => Ok(RunOutput::Scalar(geo::dating::potassium_argon_age(
            get_f(p, "ar40")?,
            get_f(p, "k40")?,
        ))),
        "uranium_lead_age" => Ok(RunOutput::Scalar(geo::dating::uranium_lead_age(
            get_f(p, "pb206")?,
            get_f(p, "u238")?,
        ))),
        "isochron_age" => Ok(RunOutput::Scalar(geo::dating::isochron_age(
            get_f(p, "slope")?,
            get_f(p, "lambda")?,
        ))),
        "fission_track_age" => Ok(RunOutput::Scalar(geo::dating::fission_track_age(
            get_f(p, "rho_s")?,
            get_f(p, "rho_i")?,
            get_f(p, "rho_d")?,
            get_f(p, "lambda")?,
        ))),
        "luminescence_dose" => Ok(RunOutput::Scalar(geo::dating::luminescence_dose(
            get_f(p, "natural_signal")?,
            get_f(p, "dose_rate")?,
        ))),
        "cosmogenic_exposure_age" => Ok(RunOutput::Scalar(geo::dating::cosmogenic_exposure_age(
            get_f(p, "concentration")?,
            get_f(p, "production_rate")?,
            get_f(p, "lambda")?,
        ))),
        "uranium_235_lead_age" => Ok(RunOutput::Scalar(geo::dating::uranium_235_lead_age(
            get_f(p, "pb207")?,
            get_f(p, "u235")?,
        ))),
        "concordia_u238_pb206" => Ok(RunOutput::Scalar(geo::dating::concordia_u238_pb206(get_f(
            p, "t",
        )?))),
        "concordia_u235_pb207" => Ok(RunOutput::Scalar(geo::dating::concordia_u235_pb207(get_f(
            p, "t",
        )?))),
        "concordia_age" => Ok(RunOutput::Scalar(geo::dating::concordia_age(
            get_f(p, "pb206_u238")?,
            get_f(p, "pb207_u235")?,
        ))),
        "thorium_232_lead_age" => Ok(RunOutput::Scalar(geo::dating::thorium_232_lead_age(
            get_f(p, "pb208")?,
            get_f(p, "th232")?,
        ))),
        "u_th_he_age" => Ok(RunOutput::Scalar(geo::dating::u_th_he_age(
            get_f(p, "he4")?,
            get_f(p, "u238")?,
            get_f(p, "u235")?,
            get_f(p, "th232")?,
        ))),
        "radiogenic_heat_production" => {
            Ok(RunOutput::Scalar(geo::dating::radiogenic_heat_production(
                get_f(p, "u238_ppm")?,
                get_f(p, "th232_ppm")?,
                get_f(p, "k40_ppm")?,
                get_f(p, "density")?,
            )))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
