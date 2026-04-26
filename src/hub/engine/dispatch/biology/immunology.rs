//! Dispatch handler for immunology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "clonal_expansion" => Ok(RunOutput::Scalar(
            bio::immunology::adaptive::clonal_expansion(
                get_f(p, "n0")?,
                get_f(p, "proliferation_rate")?,
                get_f(p, "death_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "clonal_selection_dynamics" => {
            let r = bio::immunology::adaptive::clonal_selection_dynamics(
                get_f(p, "naive")?,
                get_f(p, "effector")?,
                get_f(p, "memory")?,
                get_f(p, "antigen")?,
                get_f(p, "k_activation")?,
                get_f(p, "k_prolif")?,
                get_f(p, "k_death")?,
                get_f(p, "k_memory")?,
                get_f(p, "k_clear")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "clonal_selection_simulate" => Ok(RunOutput::Matrix(
            bio::immunology::adaptive::clonal_selection_simulate(
                get_f(p, "naive0")?,
                get_f(p, "effector0")?,
                get_f(p, "memory0")?,
                get_f(p, "antigen0")?,
                get_f(p, "k_activation")?,
                get_f(p, "k_prolif")?,
                get_f(p, "k_death")?,
                get_f(p, "k_memory")?,
                get_f(p, "k_clear")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            )
            .into_iter()
            .map(|(a, b, c, d)| vec![a, b, c, d])
            .collect(),
        )),
        "tcell_activation_threshold" => Ok(RunOutput::Boolean(
            bio::immunology::adaptive::tcell_activation_threshold(
                get_f(p, "signal")?,
                get_f(p, "costimulation")?,
                get_f(p, "threshold")?,
            ),
        )),
        "cytokine_hill_response" => Ok(RunOutput::Scalar(
            bio::immunology::adaptive::cytokine_hill_response(
                get_f(p, "cytokine")?,
                get_f(p, "ec50")?,
                get_f(p, "n")?,
            ),
        )),
        "treg_suppression" => Ok(RunOutput::Scalar(
            bio::immunology::adaptive::treg_suppression(
                get_f(p, "effector_rate")?,
                get_f(p, "treg")?,
                get_f(p, "k_supp")?,
            ),
        )),
        "memory_recall_response" => Ok(RunOutput::Scalar(
            bio::immunology::adaptive::memory_recall_response(
                get_f(p, "memory")?,
                get_f(p, "antigen")?,
                get_f(p, "k_recall")?,
                get_f(p, "fold_expansion")?,
            ),
        )),
        "somatic_affinity_maturation" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::somatic_affinity_maturation(
                get_f(p, "initial_kd")?,
                get_u(p, "rounds")?,
                get_f(p, "improvement_per_round")?,
            ),
        )),
        "antibody_titer" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::antibody_titer(
                get_f(p, "dilution_factor")?,
                get_u(p, "endpoint_dilution")?,
            ),
        )),
        "isotype_switch_probability" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::isotype_switch_probability(
                get_f(p, "cytokine_signal")?,
                get_f(p, "switch_region_accessibility")?,
                get_f(p, "aid_activity")?,
            ),
        )),
        "somatic_hypermutation_rate" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::somatic_hypermutation_rate(
                get_f(p, "aid_expression")?,
                get_f(p, "base_rate")?,
                get_f(p, "hotspot_factor")?,
            ),
        )),
        "neutralization_potency" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::neutralization_potency(
                get_f(p, "ic50")?,
                get_f(p, "virus_titer")?,
            ),
        )),
        "opsonization_index" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::opsonization_index(
                get_f(p, "antibody_bound")?,
                get_f(p, "fc_receptor_density")?,
                get_f(p, "complement_coating")?,
            ),
        )),
        "adcc_killing_rate" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::adcc_killing_rate(
                get_f(p, "antibody_density")?,
                get_f(p, "nk_cell_count")?,
                get_f(p, "target_count")?,
                get_f(p, "k_sat")?,
            ),
        )),
        "complement_cascade_c3b" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::complement_cascade_c3b(
                get_f(p, "c3")?,
                get_f(p, "convertase_activity")?,
                get_f(p, "factor_h_inhibition")?,
            ),
        )),
        "multivalent_avidity" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::multivalent_avidity(
                get_f(p, "monovalent_kd")?,
                get_u(p, "valency")?,
                get_f(p, "cooperativity")?,
            ),
        )),
        "elisa_concentration" => Ok(RunOutput::Scalar(
            bio::immunology::antibodies::elisa_concentration(
                get_f(p, "od")?,
                get_f(p, "od_max")?,
                get_f(p, "ec50")?,
                get_f(p, "hill")?,
            ),
        )),
        "b_cell_germinal_center_selection" => Ok(RunOutput::Boolean(
            bio::immunology::antibodies::b_cell_germinal_center_selection(
                get_f(p, "affinity")?,
                get_f(p, "threshold")?,
                get_f(p, "t_cell_help")?,
            ),
        )),
        "cytokine_production_rate" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::cytokine_production_rate(
                get_f(p, "stimulus")?,
                get_f(p, "cell_count")?,
                get_f(p, "max_rate")?,
                get_f(p, "ec50")?,
            ),
        )),
        "cytokine_decay" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::cytokine_decay(
                get_f(p, "concentration")?,
                get_f(p, "half_life")?,
                get_f(p, "dt")?,
            ),
        )),
        "th1_th2_balance" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::th1_th2_balance(get_f(p, "ifn_gamma")?, get_f(p, "il4")?),
        )),
        "th17_regulatory_balance" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::th17_regulatory_balance(
                get_f(p, "il17")?,
                get_f(p, "il10")?,
                get_f(p, "tgf_beta")?,
            ),
        )),
        "cytokine_storm_severity" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::cytokine_storm_severity(
                get_f(p, "il6")?,
                get_f(p, "tnf")?,
                get_f(p, "il1b")?,
                get_f(p, "threshold_il6")?,
                get_f(p, "threshold_tnf")?,
                get_f(p, "threshold_il1b")?,
            ),
        )),
        "jak_stat_signaling" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::jak_stat_signaling(
                get_f(p, "cytokine")?,
                get_f(p, "receptor_density")?,
                get_f(p, "jak_activity")?,
                get_f(p, "socs_inhibition")?,
            ),
        )),
        "nfkb_activation" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::nfkb_activation(
                get_f(p, "stimulus")?,
                get_f(p, "ikk_activity")?,
                get_f(p, "ikb_level")?,
            ),
        )),
        "chemokine_gradient" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::chemokine_gradient(
                get_f(p, "source_conc")?,
                get_f(p, "distance")?,
                get_f(p, "diffusion_coeff")?,
                get_f(p, "decay_rate")?,
            ),
        )),
        "autocrine_loop" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::autocrine_loop(
                get_f(p, "production_rate")?,
                get_f(p, "receptor_sensitivity")?,
                get_f(p, "degradation")?,
            ),
        )),
        "paracrine_signaling" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::paracrine_signaling(
                get_f(p, "source_cells")?,
                get_f(p, "target_distance")?,
                get_f(p, "diffusion")?,
                get_f(p, "production")?,
                get_f(p, "decay")?,
            ),
        )),
        "nlrp3_inflammasome_activation" => Ok(RunOutput::Scalar(
            bio::immunology::cytokines::nlrp3_inflammasome_activation(
                get_f(p, "damp_signal")?,
                get_f(p, "nlrp3")?,
                get_f(p, "asc")?,
                get_f(p, "threshold")?,
            ),
        )),
        "sir_immune" => {
            let r = bio::immunology::dynamics::sir_immune(
                get_f(p, "s")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "immune")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "k_immune")?,
                get_f(p, "k_decay")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "sir_immune_simulate" => Ok(RunOutput::Matrix(
            bio::immunology::dynamics::sir_immune_simulate(
                get_f(p, "s0")?,
                get_f(p, "i0")?,
                get_f(p, "r0")?,
                get_f(p, "immune0")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "k_immune")?,
                get_f(p, "k_decay")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            )
            .into_iter()
            .map(|(a, b, c, d)| vec![a, b, c, d])
            .collect(),
        )),
        "vaccine_efficacy" => Ok(RunOutput::Scalar(
            bio::immunology::dynamics::vaccine_efficacy(
                get_f(p, "arr_vacc")?,
                get_f(p, "arr_placebo")?,
            ),
        )),
        "herd_immunity_fraction" => Ok(RunOutput::Scalar(
            bio::immunology::dynamics::herd_immunity_fraction(get_f(p, "r0")?),
        )),
        "antibody_decay" => Ok(RunOutput::Scalar(
            bio::immunology::dynamics::antibody_decay(
                get_f(p, "ab0")?,
                get_f(p, "half_life")?,
                get_f(p, "t")?,
            ),
        )),
        "booster_response" => Ok(RunOutput::Scalar(
            bio::immunology::dynamics::booster_response(
                get_f(p, "ab_pre")?,
                get_f(p, "fold_boost")?,
                get_f(p, "decay_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "seroconversion_probability" => Ok(RunOutput::Scalar(
            bio::immunology::dynamics::seroconversion_probability(
                get_f(p, "dose")?,
                get_f(p, "ed50")?,
                get_f(p, "n")?,
            ),
        )),
        "waning_immunity" => Ok(RunOutput::Scalar(
            bio::immunology::dynamics::waning_immunity(
                get_f(p, "immune_fraction")?,
                get_f(p, "waning_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "immuno_maternal_antibody_decay" => Ok(RunOutput::Scalar(
            bio::immunology::dynamics::maternal_antibody_decay(
                get_f(p, "ab0")?,
                get_f(p, "half_life")?,
                get_f(p, "t_months")?,
            ),
        )),
        "immuno_seir_step" => {
            let r = bio::immunology::dynamics::seir_step(
                get_f(p, "s")?,
                get_f(p, "e")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "beta")?,
                get_f(p, "sigma")?,
                get_f(p, "gamma")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "antigen_antibody_binding" => Ok(RunOutput::Scalar(
            bio::immunology::innate::antigen_antibody_binding(
                get_f(p, "ab")?,
                get_f(p, "ag")?,
                get_f(p, "ka")?,
            ),
        )),
        "affinity_maturation" => Ok(RunOutput::Vector(
            bio::immunology::innate::affinity_maturation(
                get_f(p, "kd_initial")?,
                get_f(p, "mutation_rate")?,
                get_u(p, "generations")?,
            ),
        )),
        "avidity" => Ok(RunOutput::Scalar(bio::immunology::innate::avidity(
            get_f(p, "kd_monovalent")?,
            get_u(p, "n_sites")?,
        ))),
        "neutralization_curve" => Ok(RunOutput::Scalar(
            bio::immunology::innate::neutralization_curve(
                get_f(p, "dose")?,
                get_f(p, "ic50")?,
                get_f(p, "n")?,
            ),
        )),
        "complement_cascade" => Ok(RunOutput::Scalar(
            bio::immunology::innate::complement_cascade(
                get_f(p, "c0")?,
                get_f(p, "rate")?,
                get_f(p, "t")?,
            ),
        )),
        "toll_like_receptor_activation" => Ok(RunOutput::Scalar(
            bio::immunology::innate::toll_like_receptor_activation(
                get_f(p, "pamp")?,
                get_f(p, "receptor_density")?,
                get_f(p, "kd")?,
            ),
        )),
        "phagocytosis_rate" => Ok(RunOutput::Scalar(
            bio::immunology::innate::phagocytosis_rate(
                get_f(p, "pathogen")?,
                get_f(p, "phagocyte")?,
                get_f(p, "k_phag")?,
                get_f(p, "saturation")?,
            ),
        )),
        "nk_cell_killing" => Ok(RunOutput::Scalar(bio::immunology::innate::nk_cell_killing(
            get_f(p, "target_mhc")?,
            get_f(p, "mhc_threshold")?,
            get_f(p, "activating_ligand")?,
            get_f(p, "kill_rate")?,
        ))),
        "cytokine_cascade" => Ok(RunOutput::Matrix(
            bio::immunology::innate::cytokine_cascade(
                get_v(p, "initial_cytokines")?,
                get_m(p, "amplification")?,
                get_u(p, "steps")?,
            ),
        )),
        "inflammasome_activation" => Ok(RunOutput::Scalar(
            bio::immunology::innate::inflammasome_activation(
                get_f(p, "damp")?,
                get_f(p, "signal2")?,
                get_f(p, "threshold")?,
                get_f(p, "nlrp3")?,
            ),
        )),
        "maternal_antibody_decay" => Ok(RunOutput::Scalar(
            bio::immunology::dynamics::maternal_antibody_decay(
                get_f(p, "ab0")?,
                get_f(p, "half_life")?,
                get_f(p, "t_months")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
