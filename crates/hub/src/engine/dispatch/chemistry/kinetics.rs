//! Dispatch handler for chemical kinetics functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "michaelis_menten" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::michaelis_menten(
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "s")?,
            ),
        )),
        "lineweaver_burk_slope" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::lineweaver_burk_slope(get_f(p, "km")?, get_f(p, "vmax")?),
        )),
        "lineweaver_burk_intercept" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::lineweaver_burk_intercept(get_f(p, "vmax")?),
        )),
        "lindemann_unimolecular" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::lindemann_unimolecular(
                get_f(p, "k_inf")?,
                get_f(p, "k0")?,
                get_f(p, "m")?,
            ),
        )),
        "consecutive_reaction" => {
            let (a, b, c) = chem::kinetics::mechanisms::consecutive_reaction(
                get_f(p, "c0")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "t")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "reversible_first_order" => {
            let (a, b) = chem::kinetics::mechanisms::reversible_first_order(
                get_f(p, "c0")?,
                get_f(p, "kf")?,
                get_f(p, "kr")?,
                get_f(p, "t")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "parallel_reactions" => Ok(RunOutput::Vector(
            chem::kinetics::mechanisms::parallel_reactions(
                get_f(p, "c0")?,
                get_v(p, "k_values")?,
                get_f(p, "t")?,
            ),
        )),
        "steady_state_intermediate" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::steady_state_intermediate(
                get_f(p, "k1")?,
                get_f(p, "k_minus1")?,
                get_f(p, "k2")?,
                get_f(p, "a")?,
            ),
        )),
        "pre_equilibrium_rate" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::pre_equilibrium_rate(
                get_f(p, "k1")?,
                get_f(p, "k_minus1")?,
                get_f(p, "k2")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "catalytic_efficiency" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::catalytic_efficiency(get_f(p, "kcat")?, get_f(p, "km")?),
        )),
        "competitive_inhibition" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::competitive_inhibition(
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "s")?,
                get_f(p, "i")?,
                get_f(p, "ki")?,
            ),
        )),
        "uncompetitive_inhibition" => Ok(RunOutput::Scalar(
            chem::kinetics::mechanisms::uncompetitive_inhibition(
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "s")?,
                get_f(p, "i")?,
                get_f(p, "ki")?,
            ),
        )),

        "rate_constant_arrhenius" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::rate_constant_arrhenius(
                get_f(p, "a")?,
                get_f(p, "ea")?,
                get_f(p, "t")?,
            ),
        )),
        "half_life_first_order" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::half_life_first_order(get_f(p, "k")?),
        )),
        "concentration_first_order" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::concentration_first_order(
                get_f(p, "c0")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "concentration_second_order" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::concentration_second_order(
                get_f(p, "c0")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "concentration_zero_order" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::concentration_zero_order(
                get_f(p, "c0")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "rate_law" => Ok(RunOutput::Scalar(chem::kinetics::rates::rate_law(
            get_f(p, "k")?,
            get_v(p, "concentrations")?,
            get_v(p, "orders")?,
        ))),
        "activation_energy_two_temps" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::activation_energy_two_temps(
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "t1")?,
                get_f(p, "t2")?,
            ),
        )),
        "half_life_second_order" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::half_life_second_order(get_f(p, "k")?, get_f(p, "c0")?),
        )),
        "half_life_zero_order" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::half_life_zero_order(get_f(p, "k")?, get_f(p, "c0")?),
        )),
        "integrated_rate_law_nth" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::integrated_rate_law_nth(
                get_f(p, "c0")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
                get_f(p, "n")?,
            ),
        )),
        "eyring_equation" => Ok(RunOutput::Scalar(chem::kinetics::rates::eyring_equation(
            get_f(p, "kappa")?,
            get_f(p, "kb")?,
            get_f(p, "t")?,
            get_f(p, "h")?,
            get_f(p, "delta_g_dagger")?,
        ))),
        "collision_frequency" => Ok(RunOutput::Scalar(
            chem::kinetics::rates::collision_frequency(
                get_f(p, "na")?,
                get_f(p, "nb")?,
                get_f(p, "sigma")?,
                get_f(p, "t")?,
                get_f(p, "mu")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
