//! Dispatch handler for enzyme kinetics functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "competitive_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::competitive_inhibition(
                get_f(p, "substrate")?,
                get_f(p, "inhibitor")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "ki")?,
            ),
        )),
        "uncompetitive_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::uncompetitive_inhibition(
                get_f(p, "substrate")?,
                get_f(p, "inhibitor")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "ki")?,
            ),
        )),
        "mixed_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::mixed_inhibition(
                get_f(p, "substrate")?,
                get_f(p, "inhibitor")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "ki")?,
                get_f(p, "ki_prime")?,
            ),
        )),
        "noncompetitive_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::noncompetitive_inhibition(
                get_f(p, "substrate")?,
                get_f(p, "inhibitor")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "ki")?,
            ),
        )),
        "substrate_inhibition_velocity" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::substrate_inhibition_velocity(
                get_f(p, "substrate")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "ksi")?,
            ),
        )),
        "irreversible_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::irreversible_inhibition(
                get_f(p, "active_enzyme")?,
                get_f(p, "inhibitor")?,
                get_f(p, "k_inact")?,
                get_f(p, "t")?,
            ),
        )),
        "tight_binding_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::tight_binding_inhibition(
                get_f(p, "enzyme_total")?,
                get_f(p, "inhibitor_total")?,
                get_f(p, "ki_app")?,
            ),
        )),
        "ic50_to_ki" => Ok(RunOutput::Scalar(bio::enzyme::inhibition::ic50_to_ki(
            get_f(p, "ic50")?,
            get_f(p, "substrate")?,
            get_f(p, "km")?,
        ))),
        "ki_to_ic50" => Ok(RunOutput::Scalar(bio::enzyme::inhibition::ki_to_ic50(
            get_f(p, "ki")?,
            get_f(p, "substrate")?,
            get_f(p, "km")?,
        ))),
        "cheng_prusoff_uncompetitive" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::cheng_prusoff_uncompetitive(
                get_f(p, "ic50")?,
                get_f(p, "substrate")?,
                get_f(p, "km")?,
            ),
        )),
        "inhibition_constant_dixon" => Ok(RunOutput::Scalar(
            bio::enzyme::inhibition::inhibition_constant_dixon(
                get_f(p, "v_no_inhibitor")?,
                get_f(p, "v_with_inhibitor")?,
                get_f(p, "inhibitor")?,
                get_f(p, "substrate")?,
                get_f(p, "km")?,
            ),
        )),
        "michaelis_menten" => Ok(RunOutput::Scalar(bio::enzyme::kinetics::michaelis_menten(
            get_f(p, "s")?,
            get_f(p, "vmax")?,
            get_f(p, "km")?,
        ))),
        "michaelis_menten_competitive" => Ok(RunOutput::Scalar(
            bio::enzyme::kinetics::michaelis_menten_competitive(
                get_f(p, "s")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "inhibitor")?,
                get_f(p, "ki")?,
            ),
        )),
        "michaelis_menten_uncompetitive" => Ok(RunOutput::Scalar(
            bio::enzyme::kinetics::michaelis_menten_uncompetitive(
                get_f(p, "s")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "inhibitor")?,
                get_f(p, "ki")?,
            ),
        )),
        "michaelis_menten_noncompetitive" => Ok(RunOutput::Scalar(
            bio::enzyme::kinetics::michaelis_menten_noncompetitive(
                get_f(p, "s")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "inhibitor")?,
                get_f(p, "ki")?,
            ),
        )),
        "hill_equation" => Ok(RunOutput::Scalar(bio::enzyme::kinetics::hill_equation(
            get_f(p, "s")?,
            get_f(p, "vmax")?,
            get_f(p, "k")?,
            get_f(p, "n")?,
        ))),
        "lineweaver_burk" => {
            let (a, b) = bio::enzyme::kinetics::lineweaver_burk(get_v(p, "s")?, get_v(p, "v")?);
            Ok(RunOutput::Pair(a, b))
        }
        "eadie_hofstee" => {
            let (a, b) = bio::enzyme::kinetics::eadie_hofstee(get_v(p, "v")?, get_v(p, "s")?);
            Ok(RunOutput::Pair(a, b))
        }
        "kcat" => Ok(RunOutput::Scalar(bio::enzyme::kinetics::kcat(
            get_f(p, "vmax")?,
            get_f(p, "e_total")?,
        ))),
        "catalytic_efficiency" => Ok(RunOutput::Scalar(
            bio::enzyme::kinetics::catalytic_efficiency(get_f(p, "kcat_val")?, get_f(p, "km")?),
        )),
        "enzyme_kinetics_solve" => Ok(RunOutput::Matrix(
            bio::enzyme::mechanisms::enzyme_kinetics_solve(
                get_f(p, "s0")?,
                get_f(p, "e0")?,
                get_f(p, "k1")?,
                get_f(p, "k_1")?,
                get_f(p, "k2")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            )
            .into_iter()
            .map(|(a, b, c)| vec![a, b, c])
            .collect(),
        )),
        "ping_pong" => Ok(RunOutput::Scalar(bio::enzyme::mechanisms::ping_pong(
            get_f(p, "a")?,
            get_f(p, "b")?,
            get_f(p, "vmax")?,
            get_f(p, "ka")?,
            get_f(p, "kb")?,
        ))),
        "ordered_bi_bi" => Ok(RunOutput::Scalar(bio::enzyme::mechanisms::ordered_bi_bi(
            get_f(p, "a")?,
            get_f(p, "b")?,
            get_f(p, "vmax")?,
            get_f(p, "ka")?,
            get_f(p, "kb")?,
            get_f(p, "kia")?,
        ))),
        "random_bi_bi" => Ok(RunOutput::Scalar(bio::enzyme::mechanisms::random_bi_bi(
            get_f(p, "a")?,
            get_f(p, "b")?,
            get_f(p, "vmax")?,
            get_f(p, "ka")?,
            get_f(p, "kb")?,
            get_f(p, "kia")?,
            get_f(p, "kib")?,
        ))),
        "substrate_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::mechanisms::substrate_inhibition(
                get_f(p, "s")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "ki")?,
            ),
        )),
        "allosteric_enzyme" => Ok(RunOutput::Scalar(
            bio::enzyme::mechanisms::allosteric_enzyme(
                get_f(p, "s")?,
                get_f(p, "vmax")?,
                get_f(p, "k05")?,
                get_f(p, "n_hill")?,
            ),
        )),
        "covalent_modification_cycle" => Ok(RunOutput::Scalar(
            bio::enzyme::mechanisms::covalent_modification_cycle(
                get_f(p, "substrate")?,
                get_f(p, "kinase_vmax")?,
                get_f(p, "kinase_km")?,
                get_f(p, "phosphatase_vmax")?,
                get_f(p, "phosphatase_km")?,
            ),
        )),
        "enzyme_activation_energy" => Ok(RunOutput::Scalar(
            bio::enzyme::mechanisms::enzyme_activation_energy(
                get_f(p, "k_cat")?,
                get_f(p, "temperature")?,
            ),
        )),
        "suicide_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::mechanisms::suicide_inhibition(
                get_f(p, "e0")?,
                get_f(p, "inhibitor")?,
                get_f(p, "ki")?,
                get_f(p, "kinact")?,
                get_f(p, "t")?,
            ),
        )),
        "enzyme_cooperativity" => Ok(RunOutput::Scalar(
            bio::enzyme::mechanisms::enzyme_cooperativity(
                get_f(p, "substrate")?,
                get_f(p, "vmax")?,
                get_v(p, "s05")?,
                get_v(p, "weights")?,
            ),
        )),
        "arrhenius" => Ok(RunOutput::Scalar(bio::enzyme::metabolism::arrhenius(
            get_f(p, "a")?,
            get_f(p, "ea")?,
            get_f(p, "t")?,
        ))),
        "q10_factor" => Ok(RunOutput::Scalar(bio::enzyme::metabolism::q10_factor(
            get_f(p, "rate1")?,
            get_f(p, "rate2")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
        ))),
        "enzyme_metabolic_control_coefficient" => Ok(RunOutput::Scalar(
            bio::enzyme::metabolism::metabolic_control_coefficient(
                get_f(p, "flux_perturbed")?,
                get_f(p, "flux_original")?,
                get_f(p, "enzyme_perturbed")?,
                get_f(p, "enzyme_original")?,
            ),
        )),
        "gibbs_free_energy" => Ok(RunOutput::Scalar(
            bio::enzyme::metabolism::gibbs_free_energy(
                get_f(p, "delta_g0")?,
                get_f(p, "t")?,
                get_f(p, "q")?,
            ),
        )),
        "mass_action_ratio" => Ok(RunOutput::Scalar(
            bio::enzyme::metabolism::mass_action_ratio(
                get_v(p, "products")?,
                get_v(p, "reactants")?,
                get_v(p, "stoich_p")?,
                get_v(p, "stoich_r")?,
            ),
        )),
        "reaction_quotient_vs_keq" => Ok(RunOutput::Scalar(
            bio::enzyme::metabolism::reaction_quotient_vs_keq(get_f(p, "q")?, get_f(p, "keq")?),
        )),
        "flux_control_summation" => Ok(RunOutput::Scalar(
            bio::enzyme::metabolism::flux_control_summation(get_v(p, "coefficients")?),
        )),
        "elasticity_coefficient" => Ok(RunOutput::Scalar(
            bio::enzyme::metabolism::elasticity_coefficient(
                get_f(p, "rate")?,
                get_f(p, "metabolite")?,
                get_f(p, "delta_rate")?,
                get_f(p, "delta_metabolite")?,
            ),
        )),
        "supply_demand_modular" => Ok(RunOutput::Scalar(
            bio::enzyme::metabolism::supply_demand_modular(
                get_f(p, "supply_flux")?,
                get_f(p, "demand_flux")?,
                get_f(p, "linking_metabolite")?,
                get_f(p, "elasticity_supply")?,
                get_f(p, "elasticity_demand")?,
            ),
        )),
        "allosteric_monod_wyman_changeux" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::allosteric_monod_wyman_changeux(
                get_f(p, "substrate")?,
                get_f(p, "n")?,
                get_f(p, "l0")?,
                get_f(p, "kr")?,
                get_f(p, "kt")?,
            ),
        )),
        "hill_cooperativity" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::hill_cooperativity(
                get_f(p, "substrate")?,
                get_f(p, "k_half")?,
                get_f(p, "n")?,
            ),
        )),
        "phosphorylation_switch" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::phosphorylation_switch(
                get_f(p, "kinase")?,
                get_f(p, "phosphatase")?,
                get_f(p, "km_kin")?,
                get_f(p, "km_phos")?,
                get_f(p, "vmax_kin")?,
                get_f(p, "vmax_phos")?,
                get_f(p, "total_protein")?,
            ),
        )),
        "zymogen_activation" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::zymogen_activation(
                get_f(p, "zymogen")?,
                get_f(p, "activator")?,
                get_f(p, "k_act")?,
            ),
        )),
        "product_inhibition_ordered" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::product_inhibition_ordered(
                get_f(p, "substrate")?,
                get_f(p, "product")?,
                get_f(p, "vmax_f")?,
                get_f(p, "km")?,
                get_f(p, "kp")?,
            ),
        )),
        "isozyme_total_activity" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::isozyme_total_activity(
                get_v(p, "activities")?,
                get_v(p, "fractions")?,
            ),
        )),
        "temperature_activation" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::temperature_activation(
                get_f(p, "rate_ref")?,
                get_f(p, "ea")?,
                get_f(p, "t")?,
                get_f(p, "t_ref")?,
            ),
        )),
        "thermal_denaturation" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::thermal_denaturation(
                get_f(p, "activity")?,
                get_f(p, "k_denat")?,
                get_f(p, "t")?,
            ),
        )),
        "feedback_inhibition" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::feedback_inhibition(
                get_f(p, "product")?,
                get_f(p, "ki")?,
                get_f(p, "n")?,
                get_f(p, "vmax")?,
                get_f(p, "substrate")?,
                get_f(p, "km")?,
            ),
        )),
        "cascade_amplification" => Ok(RunOutput::Scalar(
            bio::enzyme::regulation::cascade_amplification(
                get_f(p, "initial_signal")?,
                get_v(p, "amplification_factors")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
