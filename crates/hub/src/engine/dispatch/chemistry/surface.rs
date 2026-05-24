//! Dispatch handler for surface chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "langmuir_isotherm" => Ok(RunOutput::Scalar(
            chem::surface::adsorption::langmuir_isotherm(
                get_f(p, "theta_max")?,
                get_f(p, "k")?,
                get_f(p, "pressure")?,
            ),
        )),
        "freundlich_isotherm" => Ok(RunOutput::Scalar(
            chem::surface::adsorption::freundlich_isotherm(
                get_f(p, "kf")?,
                get_f(p, "pressure")?,
                get_f(p, "n")?,
            ),
        )),
        "bet_isotherm" => Ok(RunOutput::Scalar(chem::surface::adsorption::bet_isotherm(
            get_f(p, "v_mono")?,
            get_f(p, "c")?,
            get_f(p, "p")?,
            get_f(p, "p0")?,
        ))),
        "temkin_isotherm" => Ok(RunOutput::Scalar(
            chem::surface::adsorption::temkin_isotherm(
                get_f(p, "rt_over_b")?,
                get_f(p, "a")?,
                get_f(p, "pressure")?,
            ),
        )),
        "langmuir_dissociative" => Ok(RunOutput::Scalar(
            chem::surface::adsorption::langmuir_dissociative(get_f(p, "k")?, get_f(p, "pressure")?),
        )),
        "bet_surface_area" => Ok(RunOutput::Scalar(
            chem::surface::adsorption::bet_surface_area(
                get_f(p, "v_mono")?,
                get_f(p, "cross_section")?,
                get_f(p, "avogadro")?,
                get_f(p, "molar_volume")?,
            ),
        )),

        "surface_tension_young" => Ok(RunOutput::Scalar(
            chem::surface::tension::surface_tension_young(
                get_f(p, "gamma_sv")?,
                get_f(p, "gamma_sl")?,
                get_f(p, "cos_theta")?,
            ),
        )),
        "contact_angle" => Ok(RunOutput::Scalar(chem::surface::tension::contact_angle(
            get_f(p, "gamma_sv")?,
            get_f(p, "gamma_sl")?,
            get_f(p, "gamma_lv")?,
        ))),
        "capillary_rise" => Ok(RunOutput::Scalar(chem::surface::tension::capillary_rise(
            get_f(p, "gamma")?,
            get_f(p, "cos_theta")?,
            get_f(p, "rho")?,
            get_f(p, "g")?,
            get_f(p, "radius")?,
        ))),
        "laplace_pressure" => Ok(RunOutput::Scalar(chem::surface::tension::laplace_pressure(
            get_f(p, "gamma")?,
            get_f(p, "r1")?,
            get_f(p, "r2")?,
        ))),
        "gibbs_adsorption" => Ok(RunOutput::Scalar(chem::surface::tension::gibbs_adsorption(
            get_f(p, "d_gamma")?,
            get_f(p, "d_ln_concentration")?,
            get_f(p, "temperature")?,
        ))),
        "spreading_coefficient" => Ok(RunOutput::Scalar(
            chem::surface::tension::spreading_coefficient(
                get_f(p, "gamma_sv")?,
                get_f(p, "gamma_lv")?,
                get_f(p, "gamma_sl")?,
            ),
        )),
        "work_of_adhesion" => Ok(RunOutput::Scalar(chem::surface::tension::work_of_adhesion(
            get_f(p, "gamma_lv")?,
            get_f(p, "cos_theta")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
