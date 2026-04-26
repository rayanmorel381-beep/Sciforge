//! Dispatch handler for ethology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "territory_size" => Ok(RunOutput::Scalar(bio::ethology::behavior::territory_size(
            get_f(p, "body_mass")?,
            get_f(p, "scaling_exponent")?,
            get_f(p, "constant")?,
        ))),
        "territory_defense_cost" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::territory_defense_cost(
                get_f(p, "perimeter")?,
                get_f(p, "intruder_rate")?,
                get_f(p, "cost_per_encounter")?,
            ),
        )),
        "boldness_shyness_continuum" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::boldness_shyness_continuum(
                get_f(p, "stimulus")?,
                get_f(p, "threshold")?,
                get_f(p, "steepness")?,
            ),
        )),
        "dilution_effect" => Ok(RunOutput::Scalar(bio::ethology::behavior::dilution_effect(
            get_f(p, "group_size")?,
        ))),
        "many_eyes_detection" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::many_eyes_detection(
                get_f(p, "individual_detection")?,
                get_f(p, "group_size")?,
            ),
        )),
        "dominance_probability" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::dominance_probability(
                get_f(p, "rating_a")?,
                get_f(p, "rating_b")?,
            ),
        )),
        "allee_effect_growth" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::allee_effect_growth(
                get_f(p, "n")?,
                get_f(p, "k")?,
                get_f(p, "r")?,
                get_f(p, "a")?,
            ),
        )),
        "predator_avoidance_flight_distance" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::predator_avoidance_flight_distance(
                get_f(p, "body_mass")?,
                get_f(p, "scaling")?,
                get_f(p, "risk_factor")?,
            ),
        )),
        "hamilton_relatedness_benefit" => Ok(RunOutput::Boolean(
            bio::ethology::behavior::hamilton_relatedness_benefit(
                get_f(p, "relatedness")?,
                get_f(p, "benefit")?,
                get_f(p, "cost")?,
            ),
        )),
        "reciprocal_altruism_threshold" => Ok(RunOutput::Boolean(
            bio::ethology::behavior::reciprocal_altruism_threshold(
                get_f(p, "benefit")?,
                get_f(p, "cost")?,
                get_f(p, "probability_future")?,
            ),
        )),
        "selfish_herd_risk" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::selfish_herd_risk(
                get_f(p, "distance_to_nearest")?,
                get_f(p, "predator_speed")?,
            ),
        )),
        "vigilance_group_tradeoff" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::vigilance_group_tradeoff(
                get_f(p, "group_size")?,
                get_f(p, "individual_scan_rate")?,
            ),
        )),
        "confusion_effect" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::confusion_effect(
                get_f(p, "group_size")?,
                get_f(p, "predator_success_solo")?,
            ),
        )),
        "mobbing_probability" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::mobbing_probability(
                get_f(p, "group_size")?,
                get_f(p, "predator_danger")?,
                get_f(p, "threshold")?,
            ),
        )),
        "learning_curve_operant" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::learning_curve_operant(
                get_f(p, "trials")?,
                get_f(p, "asymptote")?,
                get_f(p, "rate")?,
            ),
        )),
        "stimulus_generalization" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::stimulus_generalization(
                get_f(p, "distance")?,
                get_f(p, "width")?,
            ),
        )),
        "ideal_despotic_distribution" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::ideal_despotic_distribution(
                get_f(p, "rank")?,
                get_f(p, "max_rank")?,
                get_f(p, "total_resource")?,
            ),
        )),
        "aggression_cost_benefit" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::aggression_cost_benefit(
                get_f(p, "resource_value")?,
                get_f(p, "fighting_ability")?,
                get_f(p, "injury_cost")?,
            ),
        )),
        "migration_threshold" => Ok(RunOutput::Boolean(
            bio::ethology::behavior::migration_threshold(
                get_f(p, "food_current")?,
                get_f(p, "food_destination")?,
                get_f(p, "travel_cost")?,
            ),
        )),
        "information_center_benefit" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::information_center_benefit(
                get_f(p, "colony_size")?,
                get_f(p, "discovery_prob")?,
            ),
        )),
        "social_network_centrality" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::social_network_centrality(
                get_f(p, "connections")?,
                get_f(p, "max_connections")?,
            ),
        )),
        "handicap_signal_cost" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::handicap_signal_cost(
                get_f(p, "quality")?,
                get_f(p, "signal_intensity")?,
                get_f(p, "cost_coeff")?,
            ),
        )),
        "mate_choice_copying" => Ok(RunOutput::Scalar(
            bio::ethology::behavior::mate_choice_copying(
                get_f(p, "intrinsic_preference")?,
                get_f(p, "social_info")?,
                get_f(p, "weight")?,
            ),
        )),
        "signal_detection_d_prime" => Ok(RunOutput::Scalar(
            bio::ethology::communication::signal_detection_d_prime(
                get_f(p, "hit_rate")?,
                get_f(p, "false_alarm_rate")?,
            ),
        )),
        "honest_signal_handicap" => Ok(RunOutput::Scalar(
            bio::ethology::communication::honest_signal_handicap(
                get_f(p, "quality")?,
                get_f(p, "cost_per_signal")?,
                get_f(p, "benefit_per_signal")?,
            ),
        )),
        "alarm_call_kin_selection" => Ok(RunOutput::Boolean(
            bio::ethology::communication::alarm_call_kin_selection(
                get_f(p, "relatedness")?,
                get_f(p, "benefit_to_kin")?,
                get_f(p, "cost_to_caller")?,
            ),
        )),
        "mate_choice_threshold" => Ok(RunOutput::Boolean(
            bio::ethology::communication::mate_choice_threshold(
                get_f(p, "quality_assessed")?,
                get_f(p, "search_cost")?,
                get_u(p, "encounters")?,
                get_f(p, "threshold")?,
            ),
        )),
        "ritualized_contest" => Ok(RunOutput::Scalar(
            bio::ethology::communication::ritualized_contest(
                get_f(p, "size_a")?,
                get_f(p, "size_b")?,
                get_f(p, "motivation_a")?,
                get_f(p, "motivation_b")?,
            ),
        )),
        "hawk_dove_contest" => {
            let (a, b) = bio::ethology::communication::hawk_dove_contest(
                get_f(p, "v")?,
                get_f(p, "c")?,
                get_f(p, "p_hawk")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "producer_scrounger_frequency" => Ok(RunOutput::Scalar(
            bio::ethology::communication::producer_scrounger_frequency(
                get_f(p, "producer_payoff")?,
                get_f(p, "scrounger_payoff")?,
                get_f(p, "p_producer")?,
                get_f(p, "selection_strength")?,
            ),
        )),
        "territory_size_optimal" => Ok(RunOutput::Scalar(
            bio::ethology::communication::territory_size_optimal(
                get_f(p, "energy_gain_rate")?,
                get_f(p, "defense_cost_per_area")?,
            ),
        )),
        "dominance_index" => Ok(RunOutput::Scalar(
            bio::ethology::communication::dominance_index(
                get_f(p, "wins")?,
                get_f(p, "total_interactions")?,
            ),
        )),
        "optimal_diet_value" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::optimal_diet_value(
                get_f(p, "energy_gain")?,
                get_f(p, "handling_time")?,
                get_f(p, "encounter_rate")?,
            ),
        )),
        "ideal_free_distribution" => Ok(RunOutput::Vector(
            bio::ethology::foraging::ideal_free_distribution(
                get_v(p, "resource")?,
                get_f(p, "total_individuals")?,
            ),
        )),
        "hawk_dove_payoff" => {
            let (a, b) = bio::ethology::foraging::hawk_dove_payoff(
                get_f(p, "v")?,
                get_f(p, "c")?,
                get_f(p, "hawk_freq")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "ess_hawk_frequency" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::ess_hawk_frequency(get_f(p, "v")?, get_f(p, "c")?),
        )),
        "tit_for_tat_payoff" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::tit_for_tat_payoff(
                get_f(p, "r")?,
                get_f(p, "s")?,
                get_f(p, "t")?,
                get_f(p, "p")?,
                get_b(p, "opponent_cooperates")?,
            ),
        )),
        "risk_sensitive_foraging" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::risk_sensitive_foraging(
                get_f(p, "mean_gain")?,
                get_f(p, "variance")?,
                get_f(p, "risk_aversion")?,
            ),
        )),
        "central_place_foraging" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::central_place_foraging(
                get_f(p, "distance")?,
                get_f(p, "load")?,
                get_f(p, "travel_cost_per_unit")?,
                get_f(p, "gain_per_load")?,
            ),
        )),
        "producer_scrounger_game" => {
            let (a, b) = bio::ethology::foraging::producer_scrounger_game(
                get_f(p, "p_freq")?,
                get_f(p, "finder_advantage")?,
                get_f(p, "group_size")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "giving_up_density" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::giving_up_density(
                get_f(p, "metabolic_cost")?,
                get_f(p, "predation_cost")?,
                get_f(p, "missed_opportunity")?,
            ),
        )),
        "patch_residence_time" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::patch_residence_time(
                get_f(p, "gain_rate")?,
                get_f(p, "travel_time")?,
                get_f(p, "depletion_rate")?,
            ),
        )),
        "functional_response_type_ii" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::functional_response_type_ii(
                get_f(p, "prey_density")?,
                get_f(p, "attack_rate")?,
                get_f(p, "handling_time")?,
            ),
        )),
        "functional_response_type_iii" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::functional_response_type_iii(
                get_f(p, "prey_density")?,
                get_f(p, "attack_max")?,
                get_f(p, "half_sat")?,
                get_f(p, "handling_time")?,
            ),
        )),
        "starvation_risk" => Ok(RunOutput::Scalar(bio::ethology::foraging::starvation_risk(
            get_f(p, "reserves")?,
            get_f(p, "daily_cost")?,
            get_f(p, "variance")?,
        ))),
        "cache_pilferage_rate" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::cache_pilferage_rate(
                get_f(p, "competitors")?,
                get_f(p, "detection_prob")?,
                get_f(p, "cache_density")?,
            ),
        )),
        "optimal_load_size" => Ok(RunOutput::Scalar(
            bio::ethology::foraging::optimal_load_size(
                get_f(p, "distance")?,
                get_f(p, "max_load")?,
                get_f(p, "loading_rate")?,
                get_f(p, "travel_speed")?,
            ),
        )),
        "diet_breadth_threshold" => Ok(RunOutput::Integer(
            bio::ethology::foraging::diet_breadth_threshold(
                get_v(p, "energy")?,
                get_v(p, "handling")?,
                get_v(p, "encounter")?,
            ) as i64,
        )),
        "habituation" => Ok(RunOutput::Scalar(bio::ethology::learning::habituation(
            get_f(p, "response")?,
            get_u(p, "stimulus_count")?,
            get_f(p, "decay_rate")?,
        ))),
        "sensitization" => Ok(RunOutput::Scalar(bio::ethology::learning::sensitization(
            get_f(p, "response")?,
            get_f(p, "noxious_stimulus")?,
            get_f(p, "gain")?,
            get_f(p, "saturation")?,
        ))),
        "operant_conditioning_rate" => Ok(RunOutput::Scalar(
            bio::ethology::learning::operant_conditioning_rate(
                get_f(p, "reinforcement_rate")?,
                get_f(p, "response_rate")?,
                get_f(p, "k")?,
            ),
        )),
        "classical_conditioning_rescorla_wagner" => Ok(RunOutput::Scalar(
            bio::ethology::learning::classical_conditioning_rescorla_wagner(
                get_f(p, "v")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "lambda")?,
            ),
        )),
        "spatial_learning_distance" => Ok(RunOutput::Scalar(
            bio::ethology::learning::spatial_learning_distance(
                get_u(p, "trial")?,
                get_f(p, "asymptote")?,
                get_f(p, "learning_rate")?,
                get_f(p, "initial_distance")?,
            ),
        )),
        "imprinting_critical_period" => Ok(RunOutput::Scalar(
            bio::ethology::learning::imprinting_critical_period(
                get_f(p, "exposure_time")?,
                get_f(p, "peak_time")?,
                get_f(p, "width")?,
            ),
        )),
        "social_learning_transmission" => Ok(RunOutput::Scalar(
            bio::ethology::learning::social_learning_transmission(
                get_f(p, "demonstrators")?,
                get_f(p, "naive")?,
                get_f(p, "transmission_rate")?,
            ),
        )),
        "memory_retention_ebbinghaus" => Ok(RunOutput::Scalar(
            bio::ethology::learning::memory_retention_ebbinghaus(
                get_f(p, "strength")?,
                get_f(p, "time")?,
                get_f(p, "stability")?,
            ),
        )),
        "working_memory_capacity" => Ok(RunOutput::Scalar(
            bio::ethology::learning::working_memory_capacity(
                get_v(p, "items")?,
                get_u(p, "capacity")?,
            ),
        )),
        "win_stay_lose_shift" => Ok(RunOutput::Boolean(
            bio::ethology::learning::win_stay_lose_shift(
                get_f(p, "previous_outcome")?,
                get_f(p, "threshold")?,
            ),
        )),
        "prey_choice_ranking" => {
            let v = get_v(p, "prey_types")?;
            let types: Vec<(f64, f64)> = v.chunks(2).map(|c| (c[0], c[1])).collect();
            let ranked = bio::ethology::foraging::prey_choice_ranking(&types);
            let flat: Vec<f64> = ranked
                .iter()
                .flat_map(|(i, r)| vec![*i as f64, *r])
                .collect();
            Ok(RunOutput::Vector(flat))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
