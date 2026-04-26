//! Dispatch handler for genetics functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "genetic_drift_wright_fisher" => {
            let seed = get_i(p, "seed")? as u64;
            Ok(RunOutput::Vector(
                bio::genetics::drift::genetic_drift_wright_fisher(
                    get_f(p, "p")?,
                    get_u(p, "pop_size")?,
                    get_u(p, "generations")?,
                    seed,
                ),
            ))
        }
        "drift_effective_population_size" => Ok(RunOutput::Scalar(
            bio::genetics::drift::drift_effective_population_size(
                get_f(p, "n_males")?,
                get_f(p, "n_females")?,
            ),
        )),
        "effective_population_size_varying" => Ok(RunOutput::Scalar(
            bio::genetics::drift::effective_population_size_varying(get_v(p, "sizes")?),
        )),
        "heterozygosity_loss" => Ok(RunOutput::Scalar(
            bio::genetics::drift::heterozygosity_loss(get_f(p, "ne")?, get_u(p, "generations")?),
        )),
        "mutation_drift_equilibrium" => Ok(RunOutput::Scalar(
            bio::genetics::drift::mutation_drift_equilibrium(get_f(p, "ne")?, get_f(p, "mu")?),
        )),
        "fixation_probability_neutral" => Ok(RunOutput::Scalar(
            bio::genetics::drift::fixation_probability_neutral(get_f(p, "ne")?),
        )),
        "fixation_probability_selection" => Ok(RunOutput::Scalar(
            bio::genetics::drift::fixation_probability_selection(
                get_f(p, "ne")?,
                get_f(p, "s")?,
                get_f(p, "p")?,
            ),
        )),
        "fixation_time_neutral" => Ok(RunOutput::Scalar(
            bio::genetics::drift::fixation_time_neutral(get_f(p, "ne")?),
        )),
        "bottleneck_heterozygosity" => Ok(RunOutput::Scalar(
            bio::genetics::drift::bottleneck_heterozygosity(
                get_f(p, "h0")?,
                get_f(p, "n_bottleneck")?,
                get_u(p, "generations")?,
            ),
        )),
        "hardy_weinberg" => {
            let (a, b, c) = bio::genetics::equilibrium::hardy_weinberg(get_f(p, "p")?);
            Ok(RunOutput::Triple(a, b, c))
        }
        "hardy_weinberg_multi" => Ok(RunOutput::Matrix(
            bio::genetics::equilibrium::hardy_weinberg_multi(get_v(p, "freqs")?),
        )),
        "chi_squared_hw" => Ok(RunOutput::Scalar(
            bio::genetics::equilibrium::chi_squared_hw(
                get_v(p, "observed")?,
                get_f(p, "p")?,
                get_f(p, "n_total")?,
            ),
        )),
        "inbreeding_coefficient" => Ok(RunOutput::Scalar(
            bio::genetics::equilibrium::inbreeding_coefficient(
                get_f(p, "h_obs")?,
                get_f(p, "h_exp")?,
            ),
        )),
        "wahlund_effect" => Ok(RunOutput::Scalar(
            bio::genetics::equilibrium::wahlund_effect(get_v(p, "subpop_freqs")?),
        )),
        "fst" => Ok(RunOutput::Scalar(bio::genetics::equilibrium::fst(get_v(
            p,
            "subpop_freqs",
        )?))),
        "nei_genetic_distance" => Ok(RunOutput::Scalar(
            bio::genetics::equilibrium::nei_genetic_distance(
                get_v(p, "pop1_freqs")?,
                get_v(p, "pop2_freqs")?,
            ),
        )),
        "expected_heterozygosity" => Ok(RunOutput::Scalar(
            bio::genetics::equilibrium::expected_heterozygosity(get_v(p, "allele_freqs")?),
        )),
        "allele_frequency_cline" => Ok(RunOutput::Scalar(
            bio::genetics::equilibrium::allele_frequency_cline(
                get_f(p, "x")?,
                get_f(p, "center")?,
                get_f(p, "width")?,
            ),
        )),
        "effective_population_size" => Ok(RunOutput::Scalar(
            bio::genetics::equilibrium::effective_population_size(
                get_f(p, "n_males")?,
                get_f(p, "n_females")?,
            ),
        )),
        "recombination_frequency" => Ok(RunOutput::Scalar(
            bio::genetics::linkage::recombination_frequency(
                get_f(p, "recombinants")?,
                get_f(p, "total_offspring")?,
            ),
        )),
        "map_distance_kosambi" => Ok(RunOutput::Scalar(
            bio::genetics::linkage::map_distance_kosambi(get_f(p, "recombination_freq")?),
        )),
        "map_distance_haldane" => Ok(RunOutput::Scalar(
            bio::genetics::linkage::map_distance_haldane(get_f(p, "recombination_freq")?),
        )),
        "haldane_to_recombination" => Ok(RunOutput::Scalar(
            bio::genetics::linkage::haldane_to_recombination(get_f(p, "map_distance")?),
        )),
        "lod_score" => Ok(RunOutput::Scalar(bio::genetics::linkage::lod_score(
            get_f(p, "theta")?,
            get_u(p, "recombinants")?,
            get_u(p, "non_recombinants")?,
        ))),
        "three_point_cross_distance" => {
            let class_counts_v = get_v(p, "class_counts")?;
            let class_counts: [f64; 8] = [
                class_counts_v[0],
                class_counts_v[1],
                class_counts_v[2],
                class_counts_v[3],
                class_counts_v[4],
                class_counts_v[5],
                class_counts_v[6],
                class_counts_v[7],
            ];
            {
                let (a, b, c) = bio::genetics::linkage::three_point_cross_distance(&class_counts);
                Ok(RunOutput::Triple(a, b, c))
            }
        }
        "interference" => Ok(RunOutput::Scalar(bio::genetics::linkage::interference(
            get_f(p, "observed_double_co")?,
            get_f(p, "expected_double_co")?,
        ))),
        "chiasma_frequency" => Ok(RunOutput::Scalar(
            bio::genetics::linkage::chiasma_frequency(get_f(p, "recombination_freq")?),
        )),
        "synaptonemal_complex_length" => Ok(RunOutput::Scalar(
            bio::genetics::linkage::synaptonemal_complex_length(
                get_f(p, "chromosome_length_mb")?,
                get_f(p, "loop_size_kb")?,
            ),
        )),
        "genetics_coalescent_expected_time" => Ok(RunOutput::Scalar(
            bio::genetics::molecular::coalescent_expected_time(get_u(p, "n")?),
        )),
        "genetics_watterson_theta" => Ok(RunOutput::Scalar(
            bio::genetics::molecular::watterson_theta(
                get_u(p, "segregating_sites")?,
                get_u(p, "n")?,
            ),
        )),
        "fst_pairwise" => Ok(RunOutput::Scalar(bio::genetics::molecular::fst_pairwise(
            get_f(p, "ht")?,
            get_f(p, "hs")?,
        ))),
        "genetics_nucleotide_diversity" => {
            let sequences_strs = get_str(p, "sequences")?;
            let sequences_parts: Vec<&[u8]> =
                sequences_strs.split(',').map(|s| s.as_bytes()).collect();
            Ok(RunOutput::Scalar(
                bio::genetics::molecular::nucleotide_diversity(&sequences_parts),
            ))
        }
        "genetics_tajima_d" => Ok(RunOutput::Scalar(bio::genetics::molecular::tajima_d(
            get_f(p, "pi")?,
            get_f(p, "theta_w")?,
            get_u(p, "n")?,
            get_u(p, "seg_sites")?,
        ))),
        "nei_distance" => Ok(RunOutput::Scalar(bio::genetics::molecular::nei_distance(
            get_v(p, "pop1_freqs")?,
            get_v(p, "pop2_freqs")?,
        ))),
        "molecular_heterozygosity" => Ok(RunOutput::Scalar(
            bio::genetics::molecular::molecular_heterozygosity(get_v(p, "freqs")?),
        )),
        "broad_sense_heritability" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::broad_sense_heritability(
                get_f(p, "genetic_variance")?,
                get_f(p, "phenotypic_variance")?,
            ),
        )),
        "narrow_sense_heritability" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::narrow_sense_heritability(
                get_f(p, "additive_variance")?,
                get_f(p, "phenotypic_variance")?,
            ),
        )),
        "breeder_equation" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::breeder_equation(
                get_f(p, "heritability")?,
                get_f(p, "selection_differential")?,
            ),
        )),
        "selection_differential" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::selection_differential(
                get_f(p, "mean_selected")?,
                get_f(p, "mean_population")?,
            ),
        )),
        "additive_genetic_variance" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::additive_genetic_variance(
                get_f(p, "allele_freq")?,
                get_f(p, "allele_effect")?,
            ),
        )),
        "dominance_variance" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::dominance_variance(
                get_f(p, "allele_freq")?,
                get_f(p, "dominance_deviation")?,
            ),
        )),
        "qtl_effect_from_means" => {
            let (a, b) = bio::genetics::quantitative::qtl_effect_from_means(
                get_f(p, "mean_aa")?,
                get_f(p, "mean_ab")?,
                get_f(p, "mean_bb")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "phenotypic_variance_components" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::phenotypic_variance_components(
                get_f(p, "va")?,
                get_f(p, "vd")?,
                get_f(p, "ve")?,
                get_f(p, "vi")?,
            ),
        )),
        "realized_heritability" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::realized_heritability(
                get_f(p, "response")?,
                get_f(p, "selection_differential")?,
            ),
        )),
        "mid_parent_regression" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::mid_parent_regression(
                get_f(p, "parent1")?,
                get_f(p, "parent2")?,
                get_f(p, "heritability")?,
                get_f(p, "population_mean")?,
            ),
        )),
        "lander_botstein_lod" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::lander_botstein_lod(
                get_u(p, "n")?,
                get_f(p, "r_squared")?,
            ),
        )),
        "polygenic_score" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::polygenic_score(
                get_v(p, "effects")?,
                get_v(p, "genotypes")?,
            ),
        )),
        "falconer_liability_threshold" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::falconer_liability_threshold(get_f(p, "prevalence")?),
        )),
        "snp_heritability" => Ok(RunOutput::Scalar(
            bio::genetics::quantitative::snp_heritability(
                get_f(p, "beta_squared_sum")?,
                get_f(p, "variance_explained")?,
                get_f(p, "phenotypic_variance")?,
            ),
        )),
        "allele_frequency_selection" => Ok(RunOutput::Scalar(
            bio::genetics::selection::allele_frequency_selection(
                get_f(p, "p")?,
                get_f(p, "w_aa")?,
                get_f(p, "w_ab")?,
                get_f(p, "w_bb")?,
            ),
        )),
        "selection_iterate" => Ok(RunOutput::Vector(
            bio::genetics::selection::selection_iterate(
                get_f(p, "p0")?,
                get_f(p, "w_aa")?,
                get_f(p, "w_ab")?,
                get_f(p, "w_bb")?,
                get_u(p, "generations")?,
            ),
        )),
        "selection_coefficient" => Ok(RunOutput::Scalar(
            bio::genetics::selection::selection_coefficient(
                get_f(p, "w_mutant")?,
                get_f(p, "w_wildtype")?,
            ),
        )),
        "mutation_selection_balance" => Ok(RunOutput::Scalar(
            bio::genetics::selection::mutation_selection_balance(get_f(p, "mu")?, get_f(p, "s")?),
        )),
        "mutation_selection_balance_recessive" => Ok(RunOutput::Scalar(
            bio::genetics::selection::mutation_selection_balance_recessive(
                get_f(p, "mu")?,
                get_f(p, "s")?,
            ),
        )),
        "genetics_frequency_dependent_fitness" => Ok(RunOutput::Scalar(
            bio::genetics::selection::frequency_dependent_fitness(
                get_f(p, "p")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "heterozygote_advantage_equilibrium" => Ok(RunOutput::Scalar(
            bio::genetics::selection::heterozygote_advantage_equilibrium(
                get_f(p, "s")?,
                get_f(p, "t")?,
            ),
        )),
        "disruptive_selection" => Ok(RunOutput::Vector(
            bio::genetics::selection::disruptive_selection(
                get_f(p, "p")?,
                get_f(p, "w_aa")?,
                get_f(p, "w_ab")?,
                get_f(p, "w_bb")?,
                get_u(p, "generations")?,
            ),
        )),
        "truncation_selection" => Ok(RunOutput::Scalar(
            bio::genetics::selection::truncation_selection(
                get_f(p, "mean")?,
                get_f(p, "variance")?,
                get_f(p, "threshold")?,
            ),
        )),
        "response_to_selection" => Ok(RunOutput::Scalar(
            bio::genetics::selection::response_to_selection(
                get_f(p, "heritability")?,
                get_f(p, "selection_differential")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
