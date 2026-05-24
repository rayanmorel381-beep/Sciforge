//! Dispatch handler for pharmacology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "pharma_oral_bioavailability" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::oral_bioavailability(
                get_f(p, "fraction_absorbed")?,
                get_f(p, "gut_wall_extraction")?,
                get_f(p, "hepatic_extraction")?,
            ),
        )),
        "intestinal_permeability_papp" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::intestinal_permeability_papp(
                get_f(p, "amount_receiver")?,
                get_f(p, "area")?,
                get_f(p, "time")?,
                get_f(p, "donor_conc")?,
            ),
        )),
        "dissolution_noyes_whitney" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::dissolution_noyes_whitney(
                get_f(p, "diffusion_coeff")?,
                get_f(p, "surface_area")?,
                get_f(p, "cs")?,
                get_f(p, "c")?,
                get_f(p, "thickness")?,
                get_f(p, "volume")?,
            ),
        )),
        "biopharmaceutics_classification" => Ok(RunOutput::Integer(
            bio::pharmacology::absorption::biopharmaceutics_classification(
                get_b(p, "solubility_high")?,
                get_b(p, "permeability_high")?,
            ) as i64,
        )),
        "hepatic_clearance_well_stirred" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::hepatic_clearance_well_stirred(
                get_f(p, "liver_blood_flow")?,
                get_f(p, "fu")?,
                get_f(p, "cl_int")?,
            ),
        )),
        "renal_drug_clearance" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::renal_drug_clearance(
                get_f(p, "gfr")?,
                get_f(p, "fu")?,
                get_f(p, "secretion")?,
                get_f(p, "reabsorption_fraction")?,
            ),
        )),
        "protein_binding" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::protein_binding(
                get_f(p, "ka")?,
                get_f(p, "protein_conc")?,
            ),
        )),
        "apparent_volume_of_distribution" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::apparent_volume_of_distribution(
                get_f(p, "dose")?,
                get_f(p, "plasma_concentration")?,
            ),
        )),
        "compartment_distribution" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::compartment_distribution(
                get_f(p, "dose")?,
                get_f(p, "kel")?,
                get_f(p, "k12")?,
                get_f(p, "k21")?,
                get_f(p, "t")?,
            ),
        )),
        "p_glycoprotein_efflux" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::p_glycoprotein_efflux(
                get_f(p, "intracellular_conc")?,
                get_f(p, "pgp_activity")?,
                get_f(p, "km")?,
            ),
        )),
        "drug_drug_interaction_auc_ratio" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::drug_drug_interaction_auc_ratio(
                get_f(p, "inhibitor_conc")?,
                get_f(p, "ki")?,
            ),
        )),
        "cyp_induction_fold" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::cyp_induction_fold(
                get_f(p, "inducer_conc")?,
                get_f(p, "ec50")?,
                get_f(p, "emax")?,
            ),
        )),
        "competitive_displacement" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::competitive_displacement(
                get_f(p, "drug_a_bound")?,
                get_f(p, "drug_b_conc")?,
                get_f(p, "kb")?,
            ),
        )),
        "synergy_bliss_independence" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::synergy_bliss_independence(
                get_f(p, "effect_a")?,
                get_f(p, "effect_b")?,
            ),
        )),
        "loewe_combination_index" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::loewe_combination_index(
                get_f(p, "dose_a")?,
                get_f(p, "dose_a_alone")?,
                get_f(p, "dose_b")?,
                get_f(p, "dose_b_alone")?,
            ),
        )),
        "isobologram_point" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::isobologram_point(
                get_f(p, "dose_a")?,
                get_f(p, "ic50_a")?,
                get_f(p, "dose_b")?,
                get_f(p, "ic50_b")?,
            ),
        )),
        "prodrug_activation" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::prodrug_activation(
                get_f(p, "prodrug_conc")?,
                get_f(p, "enzyme_activity")?,
                get_f(p, "km")?,
                get_f(p, "activation_fraction")?,
            ),
        )),
        "drug_therapeutic_index" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::drug_therapeutic_index(
                get_f(p, "td50")?,
                get_f(p, "ed50")?,
            ),
        )),
        "loading_dose_calculation" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::loading_dose_calculation(
                get_f(p, "target_concentration")?,
                get_f(p, "volume_of_distribution")?,
                get_f(p, "bioavailability")?,
            ),
        )),
        "maintenance_dose_calculation" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::maintenance_dose_calculation(
                get_f(p, "target_concentration")?,
                get_f(p, "clearance")?,
                get_f(p, "bioavailability")?,
                get_f(p, "dosing_interval")?,
            ),
        )),
        "steady_state_accumulation" => Ok(RunOutput::Scalar(
            bio::pharmacology::drug_interactions::steady_state_accumulation(
                get_f(p, "dose")?,
                get_f(p, "half_life")?,
                get_f(p, "dosing_interval")?,
            ),
        )),
        "emax_model" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::emax_model(
                get_f(p, "e0")?,
                get_f(p, "emax")?,
                get_f(p, "c")?,
                get_f(p, "ec50")?,
            ),
        )),
        "sigmoid_emax" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::sigmoid_emax(
                get_f(p, "e0")?,
                get_f(p, "emax")?,
                get_f(p, "c")?,
                get_f(p, "ec50")?,
                get_f(p, "n")?,
            ),
        )),
        "log_logistic" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::log_logistic(
                get_f(p, "c")?,
                get_f(p, "ec50")?,
                get_f(p, "slope")?,
            ),
        )),
        "therapeutic_index" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::therapeutic_index(
                get_f(p, "td50")?,
                get_f(p, "ed50")?,
            ),
        )),
        "pharma_dose_response_hill" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::dose_response_hill(
                get_f(p, "dose")?,
                get_f(p, "dmax")?,
                get_f(p, "ec50")?,
                get_f(p, "n")?,
            ),
        )),
        "competitive_antagonism" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::competitive_antagonism(
                get_f(p, "agonist")?,
                get_f(p, "ec50")?,
                get_f(p, "antagonist")?,
                get_f(p, "kb")?,
            ),
        )),
        "schild_equation" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::schild_equation(
                get_f(p, "dose_ratio")?,
                get_f(p, "antagonist")?,
            ),
        )),
        "receptor_occupancy" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::receptor_occupancy(
                get_f(p, "l")?,
                get_f(p, "kd")?,
            ),
        )),
        "clark_equation" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::clark_equation(
                get_f(p, "l")?,
                get_f(p, "kd")?,
                get_f(p, "emax")?,
            ),
        )),
        "operational_model" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::operational_model(
                get_f(p, "l")?,
                get_f(p, "kd")?,
                get_f(p, "tau")?,
                get_f(p, "n")?,
                get_f(p, "emax")?,
            ),
        )),
        "imax_model" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::imax_model(
                get_f(p, "i0")?,
                get_f(p, "imax")?,
                get_f(p, "c")?,
                get_f(p, "ic50")?,
            ),
        )),
        "combination_index" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::combination_index(
                get_f(p, "d1")?,
                get_f(p, "dx1")?,
                get_f(p, "d2")?,
                get_f(p, "dx2")?,
            ),
        )),
        "non_competitive_antagonism" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::non_competitive_antagonism(
                get_f(p, "agonist")?,
                get_f(p, "ec50")?,
                get_f(p, "antagonist")?,
                get_f(p, "kb")?,
                get_f(p, "emax")?,
            ),
        )),
        "irreversible_antagonism" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::irreversible_antagonism(
                get_f(p, "agonist")?,
                get_f(p, "ec50")?,
                get_f(p, "fraction_remaining")?,
                get_f(p, "emax")?,
            ),
        )),
        "allosteric_modulator" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::allosteric_modulator(
                get_f(p, "agonist")?,
                get_f(p, "ec50")?,
                get_f(p, "modulator")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "km")?,
                get_f(p, "emax")?,
            ),
        )),
        "patlak_plot_slope" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::patlak_plot_slope(
                get_f(p, "plasma_integral")?,
                get_f(p, "plasma_conc")?,
                get_f(p, "tissue_conc")?,
            ),
        )),
        "two_state_receptor" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::two_state_receptor(
                get_f(p, "l")?,
                get_f(p, "kd_active")?,
                get_f(p, "kd_inactive")?,
                get_f(p, "l0")?,
            ),
        )),
        "partial_agonist_effect" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::partial_agonist_effect(
                get_f(p, "l")?,
                get_f(p, "kd")?,
                get_f(p, "intrinsic_efficacy")?,
                get_f(p, "emax")?,
            ),
        )),
        "inverse_agonist_effect" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::inverse_agonist_effect(
                get_f(p, "e0")?,
                get_f(p, "l")?,
                get_f(p, "kd")?,
                get_f(p, "neg_efficacy")?,
            ),
        )),
        "biased_agonism_ratio" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::biased_agonism_ratio(
                get_f(p, "e1")?,
                get_f(p, "ec50_1")?,
                get_f(p, "e2")?,
                get_f(p, "ec50_2")?,
            ),
        )),
        "pk_pd_effect_compartment" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::pk_pd_effect_compartment(
                get_f(p, "ce")?,
                get_f(p, "emax")?,
                get_f(p, "ec50")?,
                get_f(p, "n")?,
            ),
        )),
        "hysteresis_collapse_ke0" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::hysteresis_collapse_ke0(
                get_f(p, "plasma")?,
                get_f(p, "effect_prev")?,
                get_f(p, "ke0")?,
                get_f(p, "dt")?,
            ),
        )),
        "tolerance_factor" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::tolerance_factor(
                get_f(p, "exposure_time")?,
                get_f(p, "tolerance_rate")?,
            ),
        )),
        "one_compartment" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::one_compartment(
                get_f(p, "dose")?,
                get_f(p, "vd")?,
                get_f(p, "ke")?,
                get_f(p, "t")?,
            ),
        )),
        "one_compartment_iv_infusion" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::one_compartment_iv_infusion(
                get_f(p, "r0")?,
                get_f(p, "ke")?,
                get_f(p, "vd")?,
                get_f(p, "t")?,
                get_f(p, "t_inf")?,
            ),
        )),
        "two_compartment" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::two_compartment(
                get_f(p, "a")?,
                get_f(p, "alpha")?,
                get_f(p, "b")?,
                get_f(p, "beta")?,
                get_f(p, "t")?,
            ),
        )),
        "oral_one_compartment" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::oral_one_compartment(
                get_f(p, "f_bio")?,
                get_f(p, "dose")?,
                get_f(p, "ka")?,
                get_f(p, "ke")?,
                get_f(p, "vd")?,
                get_f(p, "t")?,
            ),
        )),
        "clearance" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::clearance(get_f(p, "ke")?, get_f(p, "vd")?),
        )),
        "half_life" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::half_life(get_f(p, "ke")?),
        )),
        "auc_iv_bolus" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::auc_iv_bolus(get_f(p, "dose")?, get_f(p, "cl")?),
        )),
        "auc_trapezoidal" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::auc_trapezoidal(
                get_v(p, "times")?,
                get_v(p, "concentrations")?,
            ),
        )),
        "bioavailability" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::bioavailability(
                get_f(p, "auc_oral")?,
                get_f(p, "dose_oral")?,
                get_f(p, "auc_iv")?,
                get_f(p, "dose_iv")?,
            ),
        )),
        "volume_of_distribution" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::volume_of_distribution(
                get_f(p, "dose")?,
                get_f(p, "c0")?,
            ),
        )),
        "steady_state_concentration" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::steady_state_concentration(
                get_f(p, "dose")?,
                get_f(p, "cl")?,
                get_f(p, "tau")?,
                get_f(p, "f_bio")?,
            ),
        )),
        "loading_dose" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::loading_dose(
                get_f(p, "css_target")?,
                get_f(p, "vd")?,
                get_f(p, "f_bio")?,
            ),
        )),
        "maintenance_dose" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::maintenance_dose(
                get_f(p, "css_target")?,
                get_f(p, "cl")?,
                get_f(p, "tau")?,
                get_f(p, "f_bio")?,
            ),
        )),
        "accumulation_factor" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::accumulation_factor(
                get_f(p, "ke")?,
                get_f(p, "tau")?,
            ),
        )),
        "tmax_oral" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::tmax_oral(get_f(p, "ka")?, get_f(p, "ke")?),
        )),
        "cmax_oral" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::cmax_oral(
                get_f(p, "f_bio")?,
                get_f(p, "dose")?,
                get_f(p, "ka")?,
                get_f(p, "ke")?,
                get_f(p, "vd")?,
            ),
        )),
        "three_compartment" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::three_compartment(
                get_f(p, "a")?,
                get_f(p, "alpha")?,
                get_f(p, "b")?,
                get_f(p, "beta")?,
                get_f(p, "c")?,
                get_f(p, "gamma")?,
                get_f(p, "t")?,
            ),
        )),
        "multiple_dose_superposition" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::multiple_dose_superposition(
                get_f(p, "dose")?,
                get_f(p, "vd")?,
                get_f(p, "ke")?,
                get_f(p, "tau")?,
                get_f(p, "t")?,
                get_u(p, "n_doses")?,
            ),
        )),
        "css_max" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::css_max(
                get_f(p, "dose")?,
                get_f(p, "vd")?,
                get_f(p, "ke")?,
                get_f(p, "tau")?,
            ),
        )),
        "css_min" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::css_min(
                get_f(p, "dose")?,
                get_f(p, "vd")?,
                get_f(p, "ke")?,
                get_f(p, "tau")?,
            ),
        )),
        "time_above_mic" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::time_above_mic(
                get_f(p, "dose")?,
                get_f(p, "vd")?,
                get_f(p, "ke")?,
                get_f(p, "mic")?,
            ),
        )),
        "hepatic_extraction_ratio" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::hepatic_extraction_ratio(
                get_f(p, "cl_hepatic")?,
                get_f(p, "q_hepatic")?,
            ),
        )),
        "well_stirred_model" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::well_stirred_model(
                get_f(p, "q_h")?,
                get_f(p, "fu")?,
                get_f(p, "cl_int")?,
            ),
        )),
        "renal_clearance" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::renal_clearance(
                get_f(p, "fraction_unbound")?,
                get_f(p, "gfr")?,
            ),
        )),
        "auc_log_trapezoidal" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::auc_log_trapezoidal(
                get_v(p, "times")?,
                get_v(p, "concentrations")?,
            ),
        )),
        "mean_residence_time" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::mean_residence_time(
                get_f(p, "aumc")?,
                get_f(p, "auc")?,
            ),
        )),
        "aumc_trapezoidal" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacokinetics::aumc_trapezoidal(
                get_v(p, "times")?,
                get_v(p, "concentrations")?,
            ),
        )),
        "flip_flop_kinetics" => Ok(RunOutput::Boolean(
            bio::pharmacology::pharmacokinetics::flip_flop_kinetics(
                get_f(p, "ka")?,
                get_f(p, "ke")?,
            ),
        )),
        "dose_response_hill" => Ok(RunOutput::Scalar(
            bio::pharmacology::pharmacodynamics::dose_response_hill(
                get_f(p, "dose")?,
                get_f(p, "dmax")?,
                get_f(p, "ec50")?,
                get_f(p, "n")?,
            ),
        )),
        "oral_bioavailability" => Ok(RunOutput::Scalar(
            bio::pharmacology::absorption::oral_bioavailability(
                get_f(p, "fraction_absorbed")?,
                get_f(p, "gut_wall_extraction")?,
                get_f(p, "hepatic_extraction")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
