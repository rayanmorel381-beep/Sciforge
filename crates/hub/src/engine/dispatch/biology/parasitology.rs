//! Dispatch handler for parasitology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "parasite_r0" => Ok(RunOutput::Scalar(
            bio::parasitology::epidemiology::parasite_r0(
                get_f(p, "beta")?,
                get_f(p, "host_density")?,
                get_f(p, "recovery_rate")?,
                get_f(p, "mortality_rate")?,
            ),
        )),
        "parasite_transmission_rate" => Ok(RunOutput::Scalar(
            bio::parasitology::epidemiology::parasite_transmission_rate(
                get_f(p, "contact_rate")?,
                get_f(p, "infectivity")?,
                get_f(p, "density_susceptible")?,
                get_f(p, "density_infected")?,
            ),
        )),
        "sir_parasite_step" => {
            let (a, b, c) = bio::parasitology::epidemiology::sir_parasite_step(
                get_f(p, "s")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "mu")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "parasite_aggregation_k" => Ok(RunOutput::Scalar(
            bio::parasitology::epidemiology::parasite_aggregation_k(
                get_f(p, "mean_burden")?,
                get_f(p, "variance_burden")?,
            ),
        )),
        "parasite_negative_binomial_prevalence" => Ok(RunOutput::Scalar(
            bio::parasitology::epidemiology::parasite_negative_binomial_prevalence(
                get_f(p, "mean_burden")?,
                get_f(p, "k")?,
            ),
        )),
        "superinfection_rate" => Ok(RunOutput::Scalar(
            bio::parasitology::epidemiology::superinfection_rate(
                get_u(p, "current_parasites")?,
                get_u(p, "max_parasites")?,
                get_f(p, "exposure_rate")?,
            ),
        )),
        "vector_borne_r0" => Ok(RunOutput::Scalar(
            bio::parasitology::epidemiology::vector_borne_r0(
                get_f(p, "mosquito_density")?,
                get_f(p, "biting_rate")?,
                get_f(p, "prob_m_to_h")?,
                get_f(p, "prob_h_to_m")?,
                get_f(p, "mosquito_mortality")?,
                get_f(p, "extrinsic_incubation")?,
                get_f(p, "recovery")?,
            ),
        )),
        "definitive_intermediate_host_cycle" => Ok(RunOutput::Scalar(
            bio::parasitology::epidemiology::definitive_intermediate_host_cycle(
                get_f(p, "cercariae_production")?,
                get_f(p, "snail_infection_rate")?,
                get_f(p, "human_exposure")?,
                get_f(p, "worm_establishment")?,
            ),
        )),
        "host_parasite_lotka_volterra" => {
            let (a, b) = bio::parasitology::host_parasite::host_parasite_lotka_volterra(
                get_f(p, "h")?,
                get_f(p, "p")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "a")?,
                get_f(p, "c")?,
                get_f(p, "d")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "anderson_may" => {
            let (a, b) = bio::parasitology::host_parasite::anderson_may(
                get_f(p, "h")?,
                get_f(p, "p")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "b")?,
                get_f(p, "d_h")?,
                get_f(p, "d_p")?,
                get_f(p, "k_aggregation")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "negative_binomial_prevalence" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::negative_binomial_prevalence(
                get_f(p, "mean_burden")?,
                get_f(p, "k")?,
            ),
        )),
        "parasite_aggregation_index" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::parasite_aggregation_index(
                get_f(p, "variance")?,
                get_f(p, "mean")?,
            ),
        )),
        "superinfection_probability" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::superinfection_probability(
                get_f(p, "exposure_rate")?,
                get_f(p, "current_burden")?,
                get_f(p, "max_burden")?,
            ),
        )),
        "basic_reproduction_number_parasite" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::basic_reproduction_number_parasite(
                get_f(p, "beta")?,
                get_f(p, "lambda")?,
                get_f(p, "h")?,
                get_f(p, "mu_p")?,
                get_f(p, "mu_h")?,
                get_f(p, "alpha")?,
            ),
        )),
        "coevolution_red_queen" => {
            let (a, b) = bio::parasitology::host_parasite::coevolution_red_queen(
                get_f(p, "host_fitness")?,
                get_f(p, "parasite_fitness")?,
                get_f(p, "arms_race_rate")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "nicholson_bailey" => {
            let (a, b) = bio::parasitology::host_parasite::nicholson_bailey(
                get_f(p, "h")?,
                get_f(p, "p")?,
                get_f(p, "r")?,
                get_f(p, "a")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "negative_binomial_zero_class" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::negative_binomial_zero_class(
                get_f(p, "mean_burden")?,
                get_f(p, "k")?,
            ),
        )),
        "parasite_induced_mortality" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::parasite_induced_mortality(
                get_f(p, "alpha")?,
                get_f(p, "burden")?,
            ),
        )),
        "acquired_immunity_reduction" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::acquired_immunity_reduction(
                get_f(p, "exposure")?,
                get_f(p, "max_immunity")?,
                get_f(p, "half_exposure")?,
            ),
        )),
        "intermediate_host_prevalence" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::intermediate_host_prevalence(
                get_f(p, "beta")?,
                get_f(p, "h2")?,
                get_f(p, "mu_l")?,
                get_f(p, "mu_h2")?,
            ),
        )),
        "cercarial_force_of_infection" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::cercarial_force_of_infection(
                get_f(p, "cercarial_density")?,
                get_f(p, "contact_rate")?,
                get_f(p, "penetration_prob")?,
            ),
        )),
        "predator_prey_parasite_manipulation" => {
            let (a, b) = bio::parasitology::host_parasite::predator_prey_parasite_manipulation(
                get_f(p, "h")?,
                get_f(p, "p")?,
                get_f(p, "prey")?,
                get_f(p, "r")?,
                get_f(p, "a")?,
                get_f(p, "manipulation_factor")?,
                get_f(p, "conversion")?,
                get_f(p, "death")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "density_dependent_fecundity" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::density_dependent_fecundity(
                get_f(p, "fecundity_max")?,
                get_f(p, "burden")?,
                get_f(p, "k_dens")?,
            ),
        )),
        "mate_probability_dioecious" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::mate_probability_dioecious(
                get_f(p, "burden")?,
                get_f(p, "k_agg")?,
            ),
        )),
        "parasite_free_equilibrium" => Ok(RunOutput::Scalar(
            bio::parasitology::host_parasite::parasite_free_equilibrium(
                get_f(p, "birth")?,
                get_f(p, "death")?,
                get_f(p, "carrying_capacity")?,
            ),
        )),
        "antigenic_variation_escape" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::antigenic_variation_escape(
                get_f(p, "immune_recognition")?,
                get_f(p, "switch_rate")?,
                get_u(p, "variants")?,
            ),
        )),
        "immune_evasion_molecular_mimicry" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::immune_evasion_molecular_mimicry(
                get_f(p, "host_molecule_similarity")?,
                get_f(p, "immune_response_base")?,
            ),
        )),
        "immunosuppression_by_parasite" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::immunosuppression_by_parasite(
                get_f(p, "il10_induction")?,
                get_f(p, "treg_expansion")?,
                get_f(p, "effector_response")?,
            ),
        )),
        "encapsulation_melanization" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::encapsulation_melanization(
                get_f(p, "hemocyte_density")?,
                get_f(p, "parasite_surface_area")?,
                get_f(p, "phenoloxidase")?,
            ),
        )),
        "acquired_immunity_buildup" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::acquired_immunity_buildup(
                get_u(p, "exposure_events")?,
                get_f(p, "max_immunity")?,
                get_f(p, "rate")?,
            ),
        )),
        "parasit_maternal_antibody_decay" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::maternal_antibody_decay(
                get_f(p, "initial_titer")?,
                get_f(p, "half_life_weeks")?,
                get_f(p, "age_weeks")?,
            ),
        )),
        "concomitant_immunity" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::concomitant_immunity(
                get_f(p, "adult_worms")?,
                get_f(p, "larval_killing_rate")?,
                get_f(p, "new_larvae")?,
            ),
        )),
        "eosinophil_response" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::eosinophil_response(
                get_f(p, "parasite_burden")?,
                get_f(p, "il5_level")?,
                get_f(p, "eosinophil_base")?,
            ),
        )),
        "granuloma_formation_rate" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::granuloma_formation_rate(
                get_f(p, "antigen_deposition")?,
                get_f(p, "macrophage_activation")?,
                get_f(p, "fibrosis_rate")?,
            ),
        )),
        "hygiene_hypothesis_index" => Ok(RunOutput::Scalar(
            bio::parasitology::immunity::hygiene_hypothesis_index(
                get_f(p, "parasite_exposure")?,
                get_f(p, "allergy_risk_base")?,
                get_f(p, "protection_factor")?,
            ),
        )),
        "parasite_virulence_tradeoff" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::parasite_virulence_tradeoff(
                get_f(p, "virulence")?,
                get_f(p, "beta_max")?,
                get_f(p, "v_half")?,
            ),
        )),
        "optimal_virulence" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::optimal_virulence(
                get_f(p, "beta_max")?,
                get_f(p, "v_half")?,
                get_f(p, "mortality_background")?,
            ),
        )),
        "immune_evasion_probability" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::immune_evasion_probability(
                get_f(p, "parasite_diversity")?,
                get_f(p, "immune_memory")?,
            ),
        )),
        "worm_burden_distribution_mean" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::worm_burden_distribution_mean(
                get_f(p, "epg")?,
                get_f(p, "fecundity")?,
            ),
        )),
        "force_of_infection" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::force_of_infection(
                get_f(p, "contact_rate")?,
                get_f(p, "environmental_contamination")?,
                get_f(p, "susceptibility")?,
            ),
        )),
        "age_intensity_profile" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::age_intensity_profile(
                get_f(p, "age")?,
                get_f(p, "peak_age")?,
                get_f(p, "max_intensity")?,
                get_f(p, "shape")?,
            ),
        )),
        "superinfection_threshold" => Ok(RunOutput::Boolean(
            bio::parasitology::virulence::superinfection_threshold(
                get_f(p, "r0_resident")?,
                get_f(p, "r0_challenger")?,
            ),
        )),
        "aggregation_parameter" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::aggregation_parameter(
                get_f(p, "mean_burden")?,
                get_f(p, "variance")?,
            ),
        )),
        "drug_resistance_spread" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::drug_resistance_spread(
                get_f(p, "sensitive_freq")?,
                get_f(p, "resistant_fitness")?,
                get_f(p, "treatment_coverage")?,
            ),
        )),
        "basic_reproduction_number_macroparasite" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::basic_reproduction_number_macroparasite(
                get_f(p, "beta")?,
                get_f(p, "lambda")?,
                get_f(p, "mu_host")?,
                get_f(p, "mu_parasite")?,
                get_f(p, "alpha")?,
            ),
        )),
        "parasit_case_fatality_rate" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::case_fatality_rate(
                get_f(p, "virulence")?,
                get_f(p, "host_resistance")?,
            ),
        )),
        "parasite_clearance_rate" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::parasite_clearance_rate(
                get_f(p, "immune_activity")?,
                get_f(p, "drug_efficacy")?,
                get_f(p, "natural_death")?,
            ),
        )),
        "morbidity_intensity" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::morbidity_intensity(
                get_f(p, "burden")?,
                get_f(p, "threshold")?,
                get_f(p, "severity_coeff")?,
            ),
        )),
        "transmission_seasonality" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::transmission_seasonality(
                get_f(p, "baseline_beta")?,
                get_f(p, "amplitude")?,
                get_f(p, "t")?,
                get_f(p, "period")?,
            ),
        )),
        "mass_drug_administration_impact" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::mass_drug_administration_impact(
                get_f(p, "prevalence")?,
                get_f(p, "coverage")?,
                get_f(p, "efficacy")?,
            ),
        )),
        "reinfection_rate" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::reinfection_rate(
                get_f(p, "force_of_infection")?,
                get_f(p, "waning_immunity")?,
                get_f(p, "time_since_treatment")?,
            ),
        )),
        "pathogen_shedding_rate" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::pathogen_shedding_rate(
                get_f(p, "burden")?,
                get_f(p, "per_parasite_shed")?,
                get_f(p, "saturation")?,
            ),
        )),
        "environmental_reservoir_decay" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::environmental_reservoir_decay(
                get_f(p, "contamination")?,
                get_f(p, "decay_rate")?,
                get_f(p, "input_rate")?,
            ),
        )),
        "host_specificity_index" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::host_specificity_index(
                get_f(p, "hosts_used")?,
                get_f(p, "hosts_available")?,
            ),
        )),
        "virulence_evolution_si" => Ok(RunOutput::Scalar(
            bio::parasitology::virulence::virulence_evolution_si(
                get_f(p, "beta")?,
                get_f(p, "alpha")?,
                get_f(p, "gamma")?,
                get_f(p, "mu")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
