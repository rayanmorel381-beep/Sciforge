//! Dispatch handler for endocrinology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "hpa_axis_cortisol" => {
            let (a, b) = bio::endocrinology::axes::hpa_axis_cortisol(
                get_f(p, "crh")?,
                get_f(p, "acth_gain")?,
                get_f(p, "cortisol_gain")?,
                get_f(p, "feedback_strength")?,
                get_f(p, "cortisol_current")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "hpg_axis_testosterone" => {
            let (a, b) = bio::endocrinology::axes::hpg_axis_testosterone(
                get_f(p, "gnrh")?,
                get_f(p, "lh_gain")?,
                get_f(p, "testosterone_gain")?,
                get_f(p, "feedback")?,
                get_f(p, "testosterone_current")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "hpt_axis_t4" => {
            let (a, b) = bio::endocrinology::axes::hpt_axis_t4(
                get_f(p, "trh")?,
                get_f(p, "tsh_gain")?,
                get_f(p, "t4_gain")?,
                get_f(p, "feedback")?,
                get_f(p, "t4_current")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "glucose_insulin_model_step" => {
            let (a, b) = bio::endocrinology::axes::glucose_insulin_model_step(
                get_f(p, "glucose")?,
                get_f(p, "insulin")?,
                get_f(p, "glucose_input")?,
                get_f(p, "dt")?,
                get_f(p, "si")?,
                get_f(p, "sg")?,
                get_f(p, "n")?,
                get_f(p, "gamma")?,
                get_f(p, "g_threshold")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "calcium_pth_feedback" => Ok(RunOutput::Scalar(
            bio::endocrinology::axes::calcium_pth_feedback(
                get_f(p, "calcium")?,
                get_f(p, "setpoint")?,
                get_f(p, "pth_max")?,
                get_f(p, "steepness")?,
            ),
        )),
        "raas_angiotensin" => Ok(RunOutput::Scalar(
            bio::endocrinology::axes::raas_angiotensin(
                get_f(p, "renin")?,
                get_f(p, "angiotensinogen")?,
                get_f(p, "ace_activity")?,
            ),
        )),
        "aldosterone_response" => Ok(RunOutput::Scalar(
            bio::endocrinology::axes::aldosterone_response(
                get_f(p, "angiotensin_ii")?,
                get_f(p, "potassium")?,
                get_f(p, "gain_ang")?,
                get_f(p, "gain_k")?,
            ),
        )),
        "growth_hormone_igf1" => Ok(RunOutput::Scalar(
            bio::endocrinology::axes::growth_hormone_igf1(
                get_f(p, "gh")?,
                get_f(p, "liver_response")?,
                get_f(p, "feedback")?,
                get_f(p, "igf1_current")?,
            ),
        )),
        "leptin_energy_feedback" => Ok(RunOutput::Scalar(
            bio::endocrinology::axes::leptin_energy_feedback(
                get_f(p, "fat_mass")?,
                get_f(p, "leptin_sensitivity")?,
                get_f(p, "energy_expenditure_base")?,
            ),
        )),
        "cortisol_awakening_response" => Ok(RunOutput::Scalar(
            bio::endocrinology::axes::cortisol_awakening_response(
                get_f(p, "basal_cortisol")?,
                get_f(p, "car_amplitude")?,
                get_f(p, "time_after_wake_min")?,
            ),
        )),
        "hormone_synthesis_rate" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::hormone_synthesis_rate(
                get_f(p, "enzyme_conc")?,
                get_f(p, "substrate")?,
                get_f(p, "km")?,
                get_f(p, "vmax")?,
            ),
        )),
        "hormone_half_life_clearance" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::hormone_half_life_clearance(
                get_f(p, "concentration")?,
                get_f(p, "half_life")?,
                get_f(p, "t")?,
            ),
        )),
        "pulsatile_release" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::pulsatile_release(
                get_f(p, "amplitude")?,
                get_f(p, "frequency")?,
                get_f(p, "t")?,
                get_f(p, "basal")?,
            ),
        )),
        "negative_feedback_loop" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::negative_feedback_loop(
                get_f(p, "setpoint")?,
                get_f(p, "current")?,
                get_f(p, "gain")?,
            ),
        )),
        "positive_feedback_loop" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::positive_feedback_loop(
                get_f(p, "stimulus")?,
                get_f(p, "hormone_level")?,
                get_f(p, "gain")?,
                get_f(p, "threshold")?,
            ),
        )),
        "receptor_saturation" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::receptor_saturation(
                get_f(p, "hormone")?,
                get_f(p, "kd")?,
                get_f(p, "receptor_total")?,
            ),
        )),
        "hormone_free_fraction" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::hormone_free_fraction(
                get_f(p, "total")?,
                get_f(p, "binding_protein")?,
                get_f(p, "kd")?,
            ),
        )),
        "circadian_hormone_profile" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::circadian_hormone_profile(
                get_f(p, "amplitude")?,
                get_f(p, "phase")?,
                get_f(p, "t_hours")?,
                get_f(p, "mesor")?,
            ),
        )),
        "steroidogenesis_rate" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::steroidogenesis_rate(
                get_f(p, "cholesterol")?,
                get_f(p, "star_protein")?,
                get_f(p, "enzyme_activity")?,
            ),
        )),
        "thyroid_hormone_conversion" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::thyroid_hormone_conversion(
                get_f(p, "t4")?,
                get_f(p, "deiodinase_activity")?,
                get_f(p, "km")?,
            ),
        )),
        "insulin_sensitivity_index" => Ok(RunOutput::Scalar(
            bio::endocrinology::hormones::insulin_sensitivity_index(
                get_f(p, "glucose")?,
                get_f(p, "insulin")?,
            ),
        )),
        "homa_ir" => Ok(RunOutput::Scalar(bio::endocrinology::hormones::homa_ir(
            get_f(p, "fasting_glucose_mmol")?,
            get_f(p, "fasting_insulin_mu_per_ml")?,
        ))),
        "homa_beta" => Ok(RunOutput::Scalar(bio::endocrinology::hormones::homa_beta(
            get_f(p, "fasting_insulin_mu_per_ml")?,
            get_f(p, "fasting_glucose_mmol")?,
        ))),
        "hormone_clearance" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::hormone_clearance(
                get_f(p, "c0")?,
                get_f(p, "half_life")?,
                get_f(p, "t")?,
            ),
        )),
        "hormone_infusion_steady_state" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::hormone_infusion_steady_state(
                get_f(p, "infusion_rate")?,
                get_f(p, "clearance_rate")?,
            ),
        )),
        "hormone_infusion_transient" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::hormone_infusion_transient(
                get_f(p, "infusion_rate")?,
                get_f(p, "clearance_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "pulsatile_secretion" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::pulsatile_secretion(
                get_f(p, "amplitude")?,
                get_f(p, "frequency")?,
                get_f(p, "phase")?,
                get_f(p, "baseline")?,
                get_f(p, "t")?,
            ),
        )),
        "endocr_negative_feedback" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::negative_feedback(
                get_f(p, "hormone_level")?,
                get_f(p, "set_point")?,
                get_f(p, "gain")?,
            ),
        )),
        "endocr_positive_feedback" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::positive_feedback(
                get_f(p, "hormone_level")?,
                get_f(p, "threshold")?,
                get_f(p, "gain")?,
                get_f(p, "max_rate")?,
            ),
        )),
        "hpa_axis_step" => {
            let (a, b, c) = bio::endocrinology::kinetics::hpa_axis_step(
                get_f(p, "crf")?,
                get_f(p, "acth")?,
                get_f(p, "cortisol")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "k3")?,
                get_f(p, "d1")?,
                get_f(p, "d2")?,
                get_f(p, "d3")?,
                get_f(p, "neg_gain")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "thyroid_axis_tsh_t4" => {
            let (a, b) = bio::endocrinology::kinetics::thyroid_axis_tsh_t4(
                get_f(p, "tsh")?,
                get_f(p, "t4")?,
                get_f(p, "trh")?,
                get_f(p, "k_stim")?,
                get_f(p, "k_inh")?,
                get_f(p, "k_prod")?,
                get_f(p, "d_tsh")?,
                get_f(p, "d_t4")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "insulin_secretion_glucose" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::insulin_secretion_glucose(
                get_f(p, "glucose")?,
                get_f(p, "beta_cell_mass")?,
                get_f(p, "km")?,
                get_f(p, "vmax")?,
            ),
        )),
        "glucose_insulin_dynamics" => {
            let (a, b) = bio::endocrinology::kinetics::glucose_insulin_dynamics(
                get_f(p, "glucose")?,
                get_f(p, "insulin")?,
                get_f(p, "gin")?,
                get_f(p, "si")?,
                get_f(p, "sg")?,
                get_f(p, "n")?,
                get_f(p, "ib")?,
                get_f(p, "gb")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "hormone_binding_to_carrier" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::hormone_binding_to_carrier(
                get_f(p, "total_hormone")?,
                get_f(p, "carrier")?,
                get_f(p, "kd")?,
            ),
        )),
        "free_hormone_fraction" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::free_hormone_fraction(
                get_f(p, "total")?,
                get_f(p, "binding_proteins")?,
                get_f(p, "kd")?,
            ),
        )),
        "cortisol_diurnal_rhythm" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::cortisol_diurnal_rhythm(
                get_f(p, "t_hours")?,
                get_f(p, "peak_amplitude")?,
                get_f(p, "nadir")?,
            ),
        )),
        "growth_hormone_pulse" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::growth_hormone_pulse(
                get_f(p, "t")?,
                get_v(p, "pulse_times")?,
                get_f(p, "amplitude")?,
                get_f(p, "half_life")?,
            ),
        )),
        "renin_angiotensin_aldosterone" => {
            let (a, b, c) = bio::endocrinology::kinetics::renin_angiotensin_aldosterone(
                get_f(p, "renin")?,
                get_f(p, "angiotensinogen")?,
                get_f(p, "ace")?,
                get_f(p, "k_renin")?,
                get_f(p, "k_ace")?,
                get_f(p, "k_aldo")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "parathyroid_calcium_response" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::parathyroid_calcium_response(
                get_f(p, "calcium")?,
                get_f(p, "set_point")?,
                get_f(p, "max_pth")?,
                get_f(p, "steepness")?,
            ),
        )),
        "leptin_secretion" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::leptin_secretion(
                get_f(p, "fat_mass")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "ghrelin_fasting_profile" => Ok(RunOutput::Scalar(
            bio::endocrinology::kinetics::ghrelin_fasting_profile(
                get_f(p, "t_since_meal")?,
                get_f(p, "peak_time")?,
                get_f(p, "amplitude")?,
                get_f(p, "baseline")?,
            ),
        )),
        "receptor_binding_fraction" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::receptor_binding_fraction(
                get_f(p, "ligand")?,
                get_f(p, "kd")?,
            ),
        )),
        "competitive_binding" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::competitive_binding(
                get_f(p, "ligand")?,
                get_f(p, "competitor")?,
                get_f(p, "kd")?,
                get_f(p, "ki")?,
            ),
        )),
        "receptor_up_regulation" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::receptor_up_regulation(
                get_f(p, "r0")?,
                get_f(p, "stimulus")?,
                get_f(p, "k_up")?,
                get_f(p, "k_deg")?,
                get_f(p, "t")?,
            ),
        )),
        "receptor_down_regulation" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::receptor_down_regulation(
                get_f(p, "r0")?,
                get_f(p, "stimulus")?,
                get_f(p, "k_down")?,
                get_f(p, "k_synth")?,
                get_f(p, "t")?,
            ),
        )),
        "endocr_dose_response_hill" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::dose_response_hill(
                get_f(p, "dose")?,
                get_f(p, "ec50")?,
                get_f(p, "emax")?,
                get_f(p, "n")?,
            ),
        )),
        "insulin_glucose_minimal_model" => {
            let (a, b) = bio::endocrinology::receptors::insulin_glucose_minimal_model(
                get_f(p, "g")?,
                get_f(p, "x")?,
                get_f(p, "insulin")?,
                get_f(p, "gb")?,
                get_f(p, "p1")?,
                get_f(p, "p2")?,
                get_f(p, "p3")?,
                get_f(p, "ib")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "receptor_internalization" => {
            let (a, b) = bio::endocrinology::receptors::receptor_internalization(
                get_f(p, "surface")?,
                get_f(p, "ligand")?,
                get_f(p, "k_intern")?,
                get_f(p, "k_recycle")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "receptor_clearance_rate" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::receptor_clearance_rate(
                get_f(p, "concentration")?,
                get_f(p, "half_life")?,
            ),
        )),
        "feedback_loop_negative" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::feedback_loop_negative(
                get_f(p, "stimulus")?,
                get_f(p, "hormone")?,
                get_f(p, "sensitivity")?,
                get_f(p, "set_point")?,
            ),
        )),
        "receptor_pulsatile_response" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::receptor_pulsatile_response(
                get_f(p, "amplitude")?,
                get_f(p, "frequency")?,
                get_f(p, "t")?,
                get_f(p, "baseline")?,
            ),
        )),
        "allosteric_modulation" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::allosteric_modulation(
                get_f(p, "ligand")?,
                get_f(p, "modulator")?,
                get_f(p, "kd")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "spare_receptor_response" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::spare_receptor_response(
                get_f(p, "ligand")?,
                get_f(p, "kd")?,
                get_f(p, "receptor_reserve")?,
            ),
        )),
        "desensitization_kinetics" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::desensitization_kinetics(
                get_f(p, "r0")?,
                get_f(p, "agonist")?,
                get_f(p, "k_desens")?,
                get_f(p, "k_resens")?,
                get_f(p, "t")?,
            ),
        )),
        "second_messenger_camp" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::second_messenger_camp(
                get_f(p, "receptor_activity")?,
                get_f(p, "k_synth")?,
                get_f(p, "k_pde")?,
                get_f(p, "basal")?,
            ),
        )),
        "ip3_calcium_release" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::ip3_calcium_release(
                get_f(p, "ip3")?,
                get_f(p, "k_release")?,
                get_f(p, "k_serca")?,
                get_f(p, "store")?,
            ),
        )),
        "receptor_dimerization" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::receptor_dimerization(
                get_f(p, "monomer")?,
                get_f(p, "kd_dimer")?,
            ),
        )),
        "beta_arrestin_recruitment" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::beta_arrestin_recruitment(
                get_f(p, "agonist")?,
                get_f(p, "receptor")?,
                get_f(p, "k_arr")?,
            ),
        )),
        "receptor_tyrosine_kinase_activation" => Ok(RunOutput::Scalar(
            bio::endocrinology::receptors::receptor_tyrosine_kinase_activation(
                get_f(p, "ligand")?,
                get_f(p, "receptor")?,
                get_f(p, "km")?,
                get_f(p, "vmax")?,
            ),
        )),
        "gpcr_g_protein_cycle" => {
            let (a, b) = bio::endocrinology::receptors::gpcr_g_protein_cycle(
                get_f(p, "active_receptor")?,
                get_f(p, "gdp_bound")?,
                get_f(p, "k_exchange")?,
                get_f(p, "k_hydrolysis")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
