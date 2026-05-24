//! Dispatch handler for paleobiology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "radiometric_age" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::radiometric_age(
                get_f(p, "parent")?,
                get_f(p, "daughter")?,
                get_f(p, "decay_constant")?,
            ),
        )),
        "half_life_to_decay_constant" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::half_life_to_decay_constant(get_f(p, "half_life")?),
        )),
        "carbon14_age" => Ok(RunOutput::Scalar(bio::paleobiology::dating::carbon14_age(
            get_f(p, "ratio_sample")?,
            get_f(p, "ratio_modern")?,
        ))),
        "extinction_rate" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::extinction_rate(
                get_f(p, "extinctions")?,
                get_f(p, "taxa_at_start")?,
                get_f(p, "interval_myr")?,
            ),
        )),
        "origination_rate" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::origination_rate(
                get_f(p, "originations")?,
                get_f(p, "taxa_at_end")?,
                get_f(p, "interval_myr")?,
            ),
        )),
        "net_diversification_rate" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::net_diversification_rate(
                get_f(p, "origination")?,
                get_f(p, "extinction")?,
            ),
        )),
        "turnover_rate" => Ok(RunOutput::Scalar(bio::paleobiology::dating::turnover_rate(
            get_f(p, "origination")?,
            get_f(p, "extinction")?,
        ))),
        "survivorship_cohort" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::survivorship_cohort(
                get_f(p, "initial")?,
                get_f(p, "extinction_rate")?,
                get_f(p, "t_myr")?,
            ),
        )),
        "standing_diversity" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::standing_diversity(
                get_f(p, "origination_rate")?,
                get_f(p, "extinction_rate")?,
                get_f(p, "d0")?,
                get_f(p, "t")?,
            ),
        )),
        "taxonomic_rate_sampling_corrected" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::taxonomic_rate_sampling_corrected(
                get_f(p, "observed_crossers")?,
                get_f(p, "singletons")?,
                get_f(p, "total")?,
            ),
        )),
        "stratigraphic_completeness" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::stratigraphic_completeness(
                get_f(p, "gaps_duration")?,
                get_f(p, "total_duration")?,
            ),
        )),
        "confidence_interval_range" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::confidence_interval_range(
                get_f(p, "known_range")?,
                get_f(p, "n_horizons")?,
                get_f(p, "confidence")?,
            ),
        )),
        "logistic_diversity" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::logistic_diversity(
                get_f(p, "d0")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "recovery_time_after_extinction" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::recovery_time_after_extinction(
                get_f(p, "pre_extinction")?,
                get_f(p, "post_extinction")?,
                get_f(p, "diversification_rate")?,
            ),
        )),
        "signor_lipps_correction" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::signor_lipps_correction(
                get_f(p, "observed_last")?,
                get_f(p, "sampling_prob")?,
                get_f(p, "n_taxa")?,
            ),
        )),
        "potassium_argon_age" => Ok(RunOutput::Scalar(
            bio::paleobiology::dating::potassium_argon_age(get_f(p, "k40")?, get_f(p, "ar40")?),
        )),
        "faith_phylogenetic_diversity" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::faith_phylogenetic_diversity(get_v(p, "branch_lengths")?),
        )),
        "mean_pairwise_distance" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::mean_pairwise_distance(
                get_v(p, "distances")?,
                get_u(p, "n_taxa")?,
            ),
        )),
        "net_relatedness_index" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::net_relatedness_index(
                get_f(p, "mpd_observed")?,
                get_f(p, "mpd_null_mean")?,
                get_f(p, "mpd_null_sd")?,
            ),
        )),
        "nearest_taxon_index" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::nearest_taxon_index(
                get_f(p, "mntd_observed")?,
                get_f(p, "mntd_null_mean")?,
                get_f(p, "mntd_null_sd")?,
            ),
        )),
        "evolutionary_distinctiveness" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::evolutionary_distinctiveness(
                get_f(p, "terminal_branch_length")?,
                get_u(p, "clade_size")?,
            ),
        )),
        "phylogenetic_species_variability" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::phylogenetic_species_variability(
                get_f(p, "species_correlation_sum")?,
                get_u(p, "n_species")?,
            ),
        )),
        "diversification_rate" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::diversification_rate(
                get_f(p, "n_extant")?,
                get_f(p, "stem_age")?,
            ),
        )),
        "lineage_through_time_expected" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::lineage_through_time_expected(
                get_f(p, "n0")?,
                get_f(p, "birth_rate")?,
                get_f(p, "death_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "gamma_statistic" => Ok(RunOutput::Scalar(
            bio::paleobiology::diversity::gamma_statistic(get_v(p, "branching_times")?),
        )),
        "background_extinction_rate" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::background_extinction_rate(
                get_f(p, "species_lost")?,
                get_f(p, "total_species")?,
                get_f(p, "time_my")?,
            ),
        )),
        "mass_extinction_magnitude" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::mass_extinction_magnitude(
                get_f(p, "species_before")?,
                get_f(p, "species_after")?,
            ),
        )),
        "recovery_time_exponential" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::recovery_time_exponential(
                get_f(p, "species_lost_fraction")?,
                get_f(p, "origination_rate")?,
            ),
        )),
        "kill_curve_severity" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::kill_curve_severity(
                get_f(p, "environmental_perturbation")?,
                get_f(p, "vulnerability")?,
                get_f(p, "threshold")?,
            ),
        )),
        "selectivity_index" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::selectivity_index(
                get_f(p, "extinction_rate_group")?,
                get_f(p, "extinction_rate_background")?,
            ),
        )),
        "origination_extinction_balance" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::origination_extinction_balance(
                get_f(p, "origination_rate")?,
                get_f(p, "extinction_rate")?,
            ),
        )),
        "survivorship_curve" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::survivorship_curve(
                get_f(p, "initial_cohort")?,
                get_f(p, "extinction_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "lazarus_taxon_probability" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::lazarus_taxon_probability(
                get_f(p, "true_extinction")?,
                get_f(p, "sampling_probability")?,
                get_f(p, "gap_duration")?,
            ),
        )),
        "signor_lipps_effect" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::signor_lipps_effect(
                get_f(p, "last_appearance")?,
                get_f(p, "sampling_interval")?,
            ),
        )),
        "biodiversity_through_time" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::biodiversity_through_time(
                get_f(p, "origination_rate")?,
                get_f(p, "extinction_rate")?,
                get_f(p, "initial_diversity")?,
                get_f(p, "t")?,
            ),
        )),
        "waiting_time_to_extinction" => Ok(RunOutput::Scalar(
            bio::paleobiology::extinction::waiting_time_to_extinction(
                get_f(p, "population_size")?,
                get_f(p, "extinction_rate")?,
            ),
        )),
        "morphological_disparity" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::morphological_disparity(get_m(p, "trait_values")?),
        )),
        "rarefied_diversity" => {
            let abundances_v = get_v(p, "abundances")?;
            let abundances: Vec<usize> = abundances_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(
                bio::paleobiology::morphology::rarefied_diversity(
                    &abundances,
                    get_u(p, "sample_size")?,
                ),
            ))
        }
        "foote_boundary_crosser_rate" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::foote_boundary_crosser_rate(
                get_f(p, "n_bt")?,
                get_f(p, "n_fl")?,
            ),
        )),
        "completeness_index" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::completeness_index(
                get_f(p, "known_intervals")?,
                get_f(p, "total_range")?,
            ),
        )),
        "lazarus_ratio" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::lazarus_ratio(
                get_f(p, "lazarus_taxa")?,
                get_f(p, "total_taxa")?,
            ),
        )),
        "body_size_cope_trend" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::body_size_cope_trend(get_v(p, "sizes")?),
        )),
        "morphospace_range" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::morphospace_range(get_v(p, "trait_values")?),
        )),
        "morphospace_volume" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::morphospace_volume(get_v(p, "ranges")?),
        )),
        "pairwise_morphological_distance" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::pairwise_morphological_distance(
                get_v(p, "a")?,
                get_v(p, "b")?,
            ),
        )),
        "evolutionary_rate_darwin" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::evolutionary_rate_darwin(
                get_f(p, "size_initial")?,
                get_f(p, "size_final")?,
                get_f(p, "time_myr")?,
            ),
        )),
        "evolutionary_rate_haldane" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::evolutionary_rate_haldane(
                get_f(p, "size_initial")?,
                get_f(p, "size_final")?,
                get_f(p, "time_generations")?,
                get_f(p, "pooled_sd")?,
            ),
        )),
        "taphonomic_bias" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::taphonomic_bias(
                get_f(p, "original_richness")?,
                get_f(p, "preservation_prob")?,
            ),
        )),
        "ghost_lineage_duration" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::ghost_lineage_duration(
                get_f(p, "first_appearance")?,
                get_f(p, "inferred_origin")?,
            ),
        )),
        "disparity_centroid_distance" => Ok(RunOutput::Scalar(
            bio::paleobiology::morphology::disparity_centroid_distance(
                get_v(p, "taxon")?,
                get_v(p, "centroid")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
