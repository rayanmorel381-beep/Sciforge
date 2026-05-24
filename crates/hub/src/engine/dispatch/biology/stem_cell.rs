//! Dispatch handler for stem cell biology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "waddington_landscape_potential" => Ok(RunOutput::Scalar(
            bio::stem_cell::differentiation::waddington_landscape_potential(
                get_f(p, "state")?,
                get_f(p, "attractor_a")?,
                get_f(p, "attractor_b")?,
                get_f(p, "barrier")?,
            ),
        )),
        "differentiation_commitment" => Ok(RunOutput::Scalar(
            bio::stem_cell::differentiation::differentiation_commitment(
                get_f(p, "transcription_factor_a")?,
                get_f(p, "transcription_factor_b")?,
                get_f(p, "hill")?,
            ),
        )),
        "lineage_progression" => {
            let (a, b) = bio::stem_cell::differentiation::lineage_progression(
                get_f(p, "progenitor")?,
                get_f(p, "differentiation_rate")?,
                get_f(p, "proliferation_rate")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "multipotency_index" => Ok(RunOutput::Scalar(
            bio::stem_cell::differentiation::multipotency_index(get_v(
                p,
                "expressed_lineage_genes",
            )?),
        )),
        "cell_fate_probability_stochastic" => Ok(RunOutput::Scalar(
            bio::stem_cell::differentiation::cell_fate_probability_stochastic(
                get_f(p, "tf_level")?,
                get_f(p, "noise")?,
                get_f(p, "threshold")?,
            ),
        )),
        "directed_differentiation_efficiency" => Ok(RunOutput::Scalar(
            bio::stem_cell::differentiation::directed_differentiation_efficiency(
                get_f(p, "target_markers")?,
                get_f(p, "total_cells")?,
            ),
        )),
        "transdifferentiation_barrier" => Ok(RunOutput::Scalar(
            bio::stem_cell::differentiation::transdifferentiation_barrier(
                get_f(p, "epigenetic_distance")?,
                get_f(p, "reprogramming_factors")?,
                get_f(p, "efficiency_base")?,
            ),
        )),
        "organoid_differentiation_layers" => Ok(RunOutput::Scalar(
            bio::stem_cell::differentiation::organoid_differentiation_layers(
                get_f(p, "time")?,
                get_f(p, "layer_rate")?,
                get_f(p, "max_layers")?,
            ),
        )),
        "terminal_differentiation_irreversibility" => Ok(RunOutput::Scalar(
            bio::stem_cell::differentiation::terminal_differentiation_irreversibility(
                get_f(p, "rb_phosphorylation")?,
                get_f(p, "cdki_level")?,
            ),
        )),
        "self_renewal_probability" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::self_renewal_probability(
                get_f(p, "symmetric_rate")?,
                get_f(p, "total_division_rate")?,
            ),
        )),
        "stem_cell_pool_dynamics" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::stem_cell_pool_dynamics(
                get_f(p, "s")?,
                get_f(p, "r")?,
                get_f(p, "d")?,
                get_f(p, "p")?,
                get_f(p, "dt")?,
            ),
        )),
        "asymmetric_division_output" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::asymmetric_division_output(
                get_f(p, "stem_cells")?,
                get_f(p, "division_rate")?,
                get_f(p, "asymmetric_fraction")?,
            ),
        )),
        "lineage_commitment" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::lineage_commitment(
                get_f(p, "signal_strength")?,
                get_f(p, "threshold")?,
                get_f(p, "hill_n")?,
            ),
        )),
        "niche_occupancy" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::niche_occupancy(
                get_f(p, "stem_cells")?,
                get_f(p, "niche_capacity")?,
            ),
        )),
        "niche_competition" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::niche_competition(
                get_f(p, "resident")?,
                get_f(p, "challenger")?,
                get_f(p, "fitness_resident")?,
                get_f(p, "fitness_challenger")?,
            ),
        )),
        "dedifferentiation_rate" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::dedifferentiation_rate(
                get_f(p, "injury_signal")?,
                get_f(p, "plasticity")?,
                get_f(p, "baseline")?,
            ),
        )),
        "stem_cell_aging" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::stem_cell_aging(
                get_f(p, "initial_pool")?,
                get_f(p, "depletion_rate")?,
                get_f(p, "age")?,
            ),
        )),
        "transit_amplifying_generations" => {
            let divisions = get_i(p, "divisions")? as u32;
            Ok(RunOutput::Scalar(
                bio::stem_cell::dynamics::transit_amplifying_generations(
                    get_f(p, "progenitor")?,
                    divisions,
                    get_f(p, "survival_per_div")?,
                ),
            ))
        }
        "quiescence_exit_rate" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::quiescence_exit_rate(
                get_f(p, "growth_factor")?,
                get_f(p, "threshold")?,
                get_f(p, "max_rate")?,
            ),
        )),
        "clonal_dominance" => Ok(RunOutput::Vector(
            bio::stem_cell::dynamics::clonal_dominance(get_v(p, "fitness")?),
        )),
        "neutral_drift_clone_survival" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::neutral_drift_clone_survival(
                get_f(p, "initial_clones")?,
                get_f(p, "time")?,
                get_f(p, "replacement_rate")?,
            ),
        )),
        "hematopoietic_hierarchy_output" => {
            let (a, b) = bio::stem_cell::dynamics::hematopoietic_hierarchy_output(
                get_f(p, "hsc")?,
                get_f(p, "mpp_rate")?,
                get_f(p, "clp_rate")?,
                get_f(p, "cmp_rate")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "telomere_shortening_per_division" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::telomere_shortening_per_division(
                get_f(p, "initial_length")?,
                get_f(p, "loss_per_division")?,
                get_f(p, "divisions")?,
            ),
        )),
        "hayflick_limit_remaining" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::hayflick_limit_remaining(
                get_f(p, "telomere_length")?,
                get_f(p, "critical_length")?,
                get_f(p, "loss_per_division")?,
            ),
        )),
        "symmetric_commitment_probability" => Ok(RunOutput::Scalar(
            bio::stem_cell::dynamics::symmetric_commitment_probability(
                get_f(p, "niche_signal")?,
                get_f(p, "k_niche")?,
            ),
        )),
        "stem_cell_niche_occupancy" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::stem_cell_niche_occupancy(
                get_f(p, "stem_cells")?,
                get_f(p, "niche_capacity")?,
                get_f(p, "adhesion_strength")?,
            ),
        )),
        "niche_signal_gradient" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::niche_signal_gradient(
                get_f(p, "source_strength")?,
                get_f(p, "distance")?,
                get_f(p, "decay_length")?,
            ),
        )),
        "quiescence_probability" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::quiescence_probability(
                get_f(p, "niche_signal")?,
                get_f(p, "threshold")?,
            ),
        )),
        "niche_asymmetric_division" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::niche_asymmetric_division(
                get_f(p, "niche_polarization")?,
                get_f(p, "cell_polarity")?,
            ),
        )),
        "hematopoietic_niche_osteoblast" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::hematopoietic_niche_osteoblast(
                get_f(p, "osteoblast_count")?,
                get_f(p, "hsc_supported")?,
                get_f(p, "max_ratio")?,
            ),
        )),
        "perivascular_niche_oxygen" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::perivascular_niche_oxygen(
                get_f(p, "distance_from_vessel")?,
                get_f(p, "vessel_po2")?,
                get_f(p, "consumption_rate")?,
                get_f(p, "diffusion")?,
            ),
        )),
        "intestinal_crypt_dynamics" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::intestinal_crypt_dynamics(
                get_f(p, "stem_cells")?,
                get_f(p, "division_rate")?,
                get_f(p, "loss_rate")?,
                get_f(p, "niche_capacity")?,
                get_f(p, "dt")?,
            ),
        )),
        "wnt_gradient_crypt" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::wnt_gradient_crypt(
                get_f(p, "position")?,
                get_f(p, "crypt_depth")?,
                get_f(p, "wnt_max")?,
            ),
        )),
        "notch_lateral_inhibition_niche" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::notch_lateral_inhibition_niche(
                get_f(p, "notch_signal")?,
                get_f(p, "delta_neighbors")?,
                get_f(p, "gain")?,
            ),
        )),
        "mesenchymal_niche_paracrine" => Ok(RunOutput::Scalar(
            bio::stem_cell::niche::mesenchymal_niche_paracrine(
                get_f(p, "mscs")?,
                get_f(p, "growth_factor_per_cell")?,
                get_f(p, "distance")?,
                get_f(p, "decay")?,
            ),
        )),
        "reprogramming_efficiency" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::reprogramming_efficiency(
                get_f(p, "oct4")?,
                get_f(p, "sox2")?,
                get_f(p, "klf4")?,
                get_f(p, "myc")?,
                get_f(p, "epigenetic_barrier")?,
            ),
        )),
        "ipsc_colony_formation" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::ipsc_colony_formation(
                get_f(p, "seeded_cells")?,
                get_f(p, "reprogramming_efficiency")?,
                get_f(p, "survival_fraction")?,
            ),
        )),
        "differentiation_cascade" => Ok(RunOutput::Vector(
            bio::stem_cell::reprogramming::differentiation_cascade(
                get_f(p, "progenitor")?,
                get_v(p, "rates")?,
            ),
        )),
        "organoid_growth" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::organoid_growth(
                get_f(p, "cells")?,
                get_f(p, "growth_rate")?,
                get_f(p, "carrying_capacity")?,
                get_f(p, "dt")?,
            ),
        )),
        "yamanaka_factor_dynamics" => {
            let r = bio::stem_cell::reprogramming::yamanaka_factor_dynamics(
                get_f(p, "oct4")?,
                get_f(p, "sox2")?,
                get_f(p, "klf4")?,
                get_f(p, "myc")?,
                get_f(p, "dt")?,
                get_f(p, "degradation")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "stochastic_reprogramming_events" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::stochastic_reprogramming_events(
                get_u(p, "cells")?,
                get_f(p, "probability_per_cell")?,
            ),
        )),
        "partial_reprogramming_state" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::partial_reprogramming_state(
                get_f(p, "methylation_age")?,
                get_u(p, "cycles")?,
                get_f(p, "reset_per_cycle")?,
            ),
        )),
        "direct_lineage_conversion" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::direct_lineage_conversion(
                get_f(p, "efficiency_base")?,
                get_v(p, "tf_combination")?,
                get_f(p, "synergy")?,
            ),
        )),
        "asymmetric_division_ratio" => {
            let (a, b) = bio::stem_cell::reprogramming::asymmetric_division_ratio(
                get_f(p, "stem_cells")?,
                get_f(p, "symmetric_prob")?,
                get_f(p, "differentiation_rate")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "epigenetic_barrier_height" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::epigenetic_barrier_height(
                get_f(p, "methylation_level")?,
                get_f(p, "histone_marks")?,
                get_f(p, "chromatin_accessibility")?,
            ),
        )),
        "crispr_activation_efficiency" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::crispr_activation_efficiency(
                get_f(p, "guide_specificity")?,
                get_f(p, "activator_strength")?,
                get_f(p, "chromatin_state")?,
            ),
        )),
        "embryoid_body_formation" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::embryoid_body_formation(
                get_f(p, "single_cells")?,
                get_f(p, "aggregation_rate")?,
                get_f(p, "min_cells_per_eb")?,
            ),
        )),
        "directed_differentiation_yield" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::directed_differentiation_yield(
                get_f(p, "input_cells")?,
                get_f(p, "protocol_efficiency")?,
                get_f(p, "purity")?,
            ),
        )),
        "maturation_index" => Ok(RunOutput::Scalar(
            bio::stem_cell::reprogramming::maturation_index(
                get_v(p, "marker_expression")?,
                get_v(p, "weights")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
