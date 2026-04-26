//! Dispatch handler for reaction engineering functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "damkohler_number" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::damkohler_number(
                get_f(p, "k")?,
                get_f(p, "tau")?,
                get_f(p, "c0")?,
                get_f(p, "order")?,
            ),
        )),
        "selectivity" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::selectivity(
                get_f(p, "r_desired")?,
                get_f(p, "r_undesired")?,
            ),
        )),
        "yield_reactor" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::yield_reactor(
                get_f(p, "moles_desired")?,
                get_f(p, "moles_reacted")?,
            ),
        )),
        "overall_selectivity" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::overall_selectivity(
                get_f(p, "moles_desired")?,
                get_f(p, "moles_all_products")?,
            ),
        )),
        "thiele_modulus" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::thiele_modulus(
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "d_eff")?,
            ),
        )),
        "effectiveness_factor_sphere" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::effectiveness_factor_sphere(get_f(p, "phi")?),
        )),
        "weisz_prater_criterion" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::weisz_prater_criterion(
                get_f(p, "r_obs")?,
                get_f(p, "r_particle")?,
                get_f(p, "d_eff")?,
                get_f(p, "c_s")?,
            ),
        )),
        "residence_time_distribution_cstr" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::residence_time_distribution_cstr(
                get_f(p, "t")?,
                get_f(p, "tau")?,
            ),
        )),
        "residence_time_distribution_pfr" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::residence_time_distribution_pfr(
                get_f(p, "t")?,
                get_f(p, "tau")?,
            ),
        )),
        "recycle_ratio_effect" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::design::recycle_ratio_effect(
                get_f(p, "x_single")?,
                get_f(p, "recycle_ratio")?,
            ),
        )),

        "cstr_volume" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::cstr_volume(
                get_f(p, "f_a0")?,
                get_f(p, "x")?,
                get_f(p, "r_a")?,
            ),
        )),
        "pfr_conversion_first_order" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::pfr_conversion_first_order(
                get_f(p, "k")?,
                get_f(p, "tau")?,
            ),
        )),
        "cstr_conversion_first_order" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::cstr_conversion_first_order(
                get_f(p, "k")?,
                get_f(p, "tau")?,
            ),
        )),
        "batch_time_first_order" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::batch_time_first_order(
                get_f(p, "k")?,
                get_f(p, "x")?,
            ),
        )),
        "batch_time_second_order" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::batch_time_second_order(
                get_f(p, "k")?,
                get_f(p, "c0")?,
                get_f(p, "x")?,
            ),
        )),
        "space_time" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::space_time(
                get_f(p, "volume")?,
                get_f(p, "volumetric_flow")?,
            ),
        )),
        "space_velocity" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::space_velocity(
                get_f(p, "volumetric_flow")?,
                get_f(p, "volume")?,
            ),
        )),
        "levenspiel_plot_area" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::levenspiel_plot_area(
                get_v(p, "fa0_over_ra")?,
                get_f(p, "dx")?,
            ),
        )),
        "cstr_series_conversion" => Ok(RunOutput::Scalar(
            chem::reaction_engineering::reactors::cstr_series_conversion(
                get_f(p, "k")?,
                get_f(p, "tau_each")?,
                get_i(p, "n")? as u32,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
