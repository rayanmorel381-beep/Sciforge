//! Dispatch handler for toxicology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "bcf_ratio" => Ok(RunOutput::Scalar(bio::toxicology::accumulation::bcf_ratio(
            get_f(p, "c_organism")?,
            get_f(p, "c_water")?,
        ))),
        "bioaccumulation_factor" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::bioaccumulation_factor(
                get_f(p, "c_organism")?,
                get_f(p, "c_environment")?,
            ),
        )),
        "biomagnification_factor" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::biomagnification_factor(
                get_f(p, "c_predator")?,
                get_f(p, "c_prey")?,
            ),
        )),
        "one_compartment_toxicokinetics" => Ok(RunOutput::Vector(
            bio::toxicology::accumulation::one_compartment_toxicokinetics(
                get_f(p, "c0")?,
                get_f(p, "k_uptake")?,
                get_f(p, "k_elim")?,
                get_f(p, "c_exposure")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "depuration_half_life" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::depuration_half_life(get_f(p, "k_elim")?),
        )),
        "toxic_units" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::toxic_units(
                get_f(p, "concentration")?,
                get_f(p, "ec50")?,
            ),
        )),
        "mixture_toxicity_concentration_addition" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::mixture_toxicity_concentration_addition(
                get_v(p, "concentrations")?,
                get_v(p, "ec50s")?,
            ),
        )),
        "haber_ct_product" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::haber_ct_product(
                get_f(p, "concentration")?,
                get_f(p, "time")?,
            ),
        )),
        "risk_quotient" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::risk_quotient(
                get_f(p, "predicted_environmental_concentration")?,
                get_f(p, "predicted_no_effect_concentration")?,
            ),
        )),
        "trophic_magnification_factor" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::trophic_magnification_factor(
                get_v(p, "concentrations")?,
                get_v(p, "trophic_levels")?,
            ),
        )),
        "two_compartment_toxicokinetics" => {
            let (a, b) = bio::toxicology::accumulation::two_compartment_toxicokinetics(
                get_f(p, "c_fast")?,
                get_f(p, "c_slow")?,
                get_f(p, "k12")?,
                get_f(p, "k21")?,
                get_f(p, "k_elim")?,
                get_f(p, "k_uptake")?,
                get_f(p, "c_exposure")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "steady_state_body_burden" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::steady_state_body_burden(
                get_f(p, "k_uptake")?,
                get_f(p, "c_exposure")?,
                get_f(p, "k_elim")?,
            ),
        )),
        "biota_sediment_accumulation_factor" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::biota_sediment_accumulation_factor(
                get_f(p, "c_organism")?,
                get_f(p, "c_sediment")?,
            ),
        )),
        "lipid_normalized_concentration" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::lipid_normalized_concentration(
                get_f(p, "c_tissue")?,
                get_f(p, "lipid_fraction")?,
            ),
        )),
        "fugacity_ratio" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::fugacity_ratio(
                get_f(p, "c_organism")?,
                get_f(p, "c_environment")?,
                get_f(p, "k_ow")?,
            ),
        )),
        "elimination_rate_from_depuration" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::elimination_rate_from_depuration(
                get_f(p, "c_start")?,
                get_f(p, "c_end")?,
                get_f(p, "time")?,
            ),
        )),
        "critical_body_residue" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::critical_body_residue(
                get_f(p, "lc50")?,
                get_f(p, "bcf")?,
            ),
        )),
        "dietary_uptake_efficiency" => Ok(RunOutput::Scalar(
            bio::toxicology::accumulation::dietary_uptake_efficiency(
                get_f(p, "assimilation")?,
                get_f(p, "feeding_rate")?,
                get_f(p, "body_weight")?,
            ),
        )),
        "ld50_probit" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::ld50_probit(
                get_v(p, "doses")?,
                get_v(p, "responses")?,
                get_v(p, "totals")?,
            ),
        )),
        "dose_response_loglogistic" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::dose_response_loglogistic(
                get_f(p, "dose")?,
                get_f(p, "ec50")?,
                get_f(p, "slope")?,
                get_f(p, "bottom")?,
                get_f(p, "top")?,
            ),
        )),
        "therapeutic_window" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::therapeutic_window(
                get_f(p, "ec50")?,
                get_f(p, "td50")?,
            ),
        )),
        "margin_of_safety" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::margin_of_safety(get_f(p, "td01")?, get_f(p, "ed99")?),
        )),
        "benchmark_dose" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::benchmark_dose(
                get_f(p, "ec_target")?,
                get_f(p, "ec50")?,
                get_f(p, "slope")?,
            ),
        )),
        "haber_rule" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::haber_rule(
                get_f(p, "concentration")?,
                get_f(p, "time")?,
                get_f(p, "n")?,
            ),
        )),
        "bioconcentration_factor" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::bioconcentration_factor(
                get_f(p, "concentration_organism")?,
                get_f(p, "concentration_water")?,
            ),
        )),
        "reference_dose" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::reference_dose(
                get_f(p, "noael")?,
                get_v(p, "uncertainty_factors")?,
            ),
        )),
        "hormesis_model" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::hormesis_model(
                get_f(p, "dose")?,
                get_f(p, "max_stimulation")?,
                get_f(p, "ec50_stimulation")?,
                get_f(p, "ec50_inhibition")?,
                get_f(p, "slope")?,
            ),
        )),
        "weibull_dose_response" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::weibull_dose_response(
                get_f(p, "dose")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "multistage_cancer_model" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::multistage_cancer_model(
                get_f(p, "dose")?,
                get_v(p, "coefficients")?,
            ),
        )),
        "safety_factor_method" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::safety_factor_method(
                get_f(p, "noael")?,
                get_f(p, "interspecies")?,
                get_f(p, "intraspecies")?,
            ),
        )),
        "acute_toxicity_ratio" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::acute_toxicity_ratio(
                get_f(p, "lc50_48h")?,
                get_f(p, "environmental_conc")?,
            ),
        )),
        "species_sensitivity_distribution" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::species_sensitivity_distribution(
                get_f(p, "log_hc5")?,
                get_f(p, "sigma")?,
                get_f(p, "z_05")?,
            ),
        )),
        "no_observed_adverse_effect_concentration" => Ok(RunOutput::Integer(
            bio::toxicology::dose_response::no_observed_adverse_effect_concentration(
                get_v(p, "responses")?,
                get_v(p, "controls")?,
            )
            .map(|x| x as i64)
            .unwrap_or(-1),
        )),
        "dose_addition_mixture" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::dose_addition_mixture(
                get_v(p, "concentrations")?,
                get_v(p, "ec50s")?,
            ),
        )),
        "independent_action_mixture" => Ok(RunOutput::Scalar(
            bio::toxicology::dose_response::independent_action_mixture(
                get_v(p, "concentrations")?,
                get_v(p, "ec50s")?,
                get_v(p, "slopes")?,
            ),
        )),
        "ssd_rank" => Ok(RunOutput::Scalar(bio::toxicology::ecotoxicology::ssd_rank(
            get_v(p, "concentrations")?,
            get_f(p, "test_concentration")?,
        ))),
        "bcf" => Ok(RunOutput::Scalar(bio::toxicology::ecotoxicology::bcf(
            get_f(p, "concentration_organism")?,
            get_f(p, "concentration_water")?,
        ))),
        "baf" => Ok(RunOutput::Scalar(bio::toxicology::ecotoxicology::baf(
            get_f(p, "concentration_organism")?,
            get_f(p, "concentration_environment")?,
        ))),
        "bmf" => Ok(RunOutput::Scalar(bio::toxicology::ecotoxicology::bmf(
            get_f(p, "concentration_predator")?,
            get_f(p, "concentration_prey")?,
        ))),
        "lc50_probit" => Ok(RunOutput::Scalar(
            bio::toxicology::ecotoxicology::lc50_probit(
                get_f(p, "log_concentration")?,
                get_f(p, "slope")?,
                get_f(p, "intercept")?,
            ),
        )),
        "environmental_half_life" => Ok(RunOutput::Scalar(
            bio::toxicology::ecotoxicology::environmental_half_life(get_f(p, "k_deg")?),
        )),
        "fugacity_level_one" => Ok(RunOutput::Scalar(
            bio::toxicology::ecotoxicology::fugacity_level_one(
                get_f(p, "total_mass")?,
                get_v(p, "z_values")?,
                get_v(p, "volumes")?,
            ),
        )),
        "predicted_no_effect_concentration" => Ok(RunOutput::Scalar(
            bio::toxicology::ecotoxicology::predicted_no_effect_concentration(
                get_f(p, "ec50")?,
                get_f(p, "assessment_factor")?,
            ),
        )),
        "tox_trophic_transfer_efficiency" => Ok(RunOutput::Scalar(
            bio::toxicology::ecotoxicology::trophic_transfer_efficiency(
                get_f(p, "assimilated")?,
                get_f(p, "ingested")?,
            ),
        )),
        "acute_toxic_unit" => Ok(RunOutput::Scalar(
            bio::toxicology::ecotoxicology::acute_toxic_unit(
                get_f(p, "concentration")?,
                get_f(p, "lc50")?,
            ),
        )),
        "tox_oxidative_stress_index" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::oxidative_stress_index(
                get_f(p, "ros_production")?,
                get_f(p, "antioxidant_capacity")?,
            ),
        )),
        "dna_adduct_formation_rate" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::dna_adduct_formation_rate(
                get_f(p, "reactive_metabolite_conc")?,
                get_f(p, "dna_conc")?,
                get_f(p, "k_adduct")?,
            ),
        )),
        "tox_dose_response_hill" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::dose_response_hill(
                get_f(p, "dose")?,
                get_f(p, "emax")?,
                get_f(p, "ec50")?,
                get_f(p, "n")?,
            ),
        )),
        "bmd_estimate" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::bmd_estimate(
                get_f(p, "background")?,
                get_f(p, "bmr")?,
                get_f(p, "slope")?,
            ),
        )),
        "hepatotoxicity_clearance_ratio" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::hepatotoxicity_clearance_ratio(
                get_f(p, "metabolite_formation")?,
                get_f(p, "metabolite_detox")?,
            ),
        )),
        "receptor_mediated_toxicity" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::receptor_mediated_toxicity(
                get_f(p, "ligand")?,
                get_f(p, "receptor_total")?,
                get_f(p, "kd")?,
            ),
        )),
        "safety_margin" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::safety_margin(
                get_f(p, "noael")?,
                get_f(p, "human_exposure")?,
            ),
        )),
        "allometric_dose_scaling" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::allometric_dose_scaling(
                get_f(p, "animal_dose")?,
                get_f(p, "animal_weight")?,
                get_f(p, "human_weight")?,
            ),
        )),
        "cytotoxicity_viability" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::cytotoxicity_viability(
                get_f(p, "live_cells")?,
                get_f(p, "total_cells")?,
            ),
        )),
        "genotoxicity_micronucleus_rate" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::genotoxicity_micronucleus_rate(
                get_f(p, "micronuclei")?,
                get_f(p, "cells_scored")?,
            ),
        )),
        "ames_mutagenicity_ratio" => Ok(RunOutput::Scalar(
            bio::toxicology::mechanisms::ames_mutagenicity_ratio(
                get_f(p, "revertants_treated")?,
                get_f(p, "revertants_control")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
