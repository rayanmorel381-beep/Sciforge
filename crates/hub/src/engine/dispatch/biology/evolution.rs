//! Dispatch handler for evolution functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "adaptation_rate" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::adaptation_rate(
                get_f(p, "selection_coefficient")?,
                get_f(p, "mutation_rate")?,
                get_f(p, "population_size")?,
            ),
        )),
        "fisher_geometric_adaptation" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::fisher_geometric_adaptation(
                get_v(p, "phenotype")?,
                get_v(p, "optimum")?,
            ),
        )),
        "adaptive_walk_progress" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::adaptive_walk_progress(
                get_f(p, "current_fitness")?,
                get_f(p, "max_fitness")?,
                get_f(p, "beneficial_rate")?,
                get_u(p, "step")?,
            ),
        )),
        "beneficial_mutation_fixation_probability" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::beneficial_mutation_fixation_probability(
                get_f(p, "s")?,
                get_f(p, "ne")?,
            ),
        )),
        "phenotypic_plasticity" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::phenotypic_plasticity(
                get_f(p, "genotype_value")?,
                get_f(p, "environment")?,
                get_f(p, "reaction_norm_slope")?,
            ),
        )),
        "baldwin_effect" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::baldwin_effect(
                get_f(p, "learning_rate")?,
                get_f(p, "genetic_assimilation")?,
                get_u(p, "generations")?,
            ),
        )),
        "red_queen_coevolution" => {
            let (a, b) = bio::evolution::adaptation::red_queen_coevolution(
                get_f(p, "host_fitness")?,
                get_f(p, "parasite_fitness")?,
                get_f(p, "host_adapt_rate")?,
                get_f(p, "parasite_adapt_rate")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "environmental_gradient_selection" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::environmental_gradient_selection(
                get_f(p, "position")?,
                get_f(p, "optimum_slope")?,
                get_f(p, "selection_width")?,
                get_f(p, "phenotype")?,
            ),
        )),
        "frequency_dependent_selection" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::frequency_dependent_selection(
                get_f(p, "frequency")?,
                get_f(p, "baseline_fitness")?,
                get_f(p, "fd_coefficient")?,
            ),
        )),
        "adaptive_radiation_rate" => Ok(RunOutput::Scalar(
            bio::evolution::adaptation::adaptive_radiation_rate(
                get_u(p, "niche_count")?,
                get_u(p, "occupied")?,
                get_f(p, "diversification_rate")?,
            ),
        )),
        "fitness_landscape_nk" => {
            let genotype = get_str(p, "genotype")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::evolution::fitness::fitness_landscape_nk(
                    genotype,
                    get_u(p, "k")?,
                    get_m(p, "contributions")?,
                ),
            ))
        }
        "fitness_landscape_additive" => {
            let genotype = get_str(p, "genotype")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::evolution::fitness::fitness_landscape_additive(genotype, get_v(p, "effects")?),
            ))
        }
        "fisher_geometric_model" => Ok(RunOutput::Scalar(
            bio::evolution::fitness::fisher_geometric_model(
                get_f(p, "distance")?,
                get_u(p, "n_dimensions")?,
            ),
        )),
        "mutation_step_probability" => Ok(RunOutput::Scalar(
            bio::evolution::fitness::mutation_step_probability(
                get_f(p, "distance")?,
                get_f(p, "step_size")?,
                get_u(p, "n_dim")?,
            ),
        )),
        "adaptive_walk" => Ok(RunOutput::Vector(bio::evolution::fitness::adaptive_walk(
            get_f(p, "distance0")?,
            get_f(p, "step_size")?,
            get_u(p, "n_dim")?,
            get_u(p, "max_steps")?,
        ))),
        "epistasis" => Ok(RunOutput::Scalar(bio::evolution::fitness::epistasis(
            get_f(p, "w_ab")?,
            get_f(p, "w_a")?,
            get_f(p, "w_b")?,
            get_f(p, "w_ref")?,
        ))),
        "evolution_frequency_dependent_fitness" => Ok(RunOutput::Scalar(
            bio::evolution::fitness::frequency_dependent_fitness(
                get_f(p, "freq")?,
                get_f(p, "advantage_rare")?,
            ),
        )),
        "density_dependent_fitness" => Ok(RunOutput::Scalar(
            bio::evolution::fitness::density_dependent_fitness(
                get_f(p, "population")?,
                get_f(p, "carrying_capacity")?,
                get_f(p, "r_max")?,
            ),
        )),
        "wrightian_fitness" => {
            let genotype_v = get_v(p, "genotype")?;
            let genotype: Vec<bool> = genotype_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::Scalar(
                bio::evolution::fitness::wrightian_fitness(
                    &genotype,
                    get_v(p, "loci_effects")?,
                    get_v(p, "dominance")?,
                ),
            ))
        }
        "substitution_rate" => Ok(RunOutput::Scalar(
            bio::evolution::molecular::substitution_rate(
                get_f(p, "mu")?,
                get_f(p, "ne")?,
                get_f(p, "s")?,
            ),
        )),
        "evolution_dn_ds_ratio" => Ok(RunOutput::Scalar(bio::evolution::molecular::dn_ds_ratio(
            get_f(p, "dn")?,
            get_f(p, "ds")?,
        ))),
        "evolution_molecular_clock_rate" => Ok(RunOutput::Scalar(
            bio::evolution::molecular::molecular_clock_rate(
                get_f(p, "substitutions")?,
                get_f(p, "divergence_time")?,
            ),
        )),
        "coalescent_time_pair" => Ok(RunOutput::Scalar(
            bio::evolution::molecular::coalescent_time_pair(get_f(p, "ne")?),
        )),
        "expected_segregating_sites" => Ok(RunOutput::Scalar(
            bio::evolution::molecular::expected_segregating_sites(
                get_f(p, "theta")?,
                get_u(p, "n")?,
            ),
        )),
        "watterson_estimator" => Ok(RunOutput::Scalar(
            bio::evolution::molecular::watterson_estimator(get_u(p, "seg_sites")?, get_u(p, "n")?),
        )),
        "mcdonald_kreitman" => Ok(RunOutput::Scalar(
            bio::evolution::molecular::mcdonald_kreitman(
                get_f(p, "dn")?,
                get_f(p, "ds")?,
                get_f(p, "pn")?,
                get_f(p, "ps")?,
            ),
        )),
        "neutral_substitution_rate" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::neutral_substitution_rate(get_f(p, "mutation_rate")?),
        )),
        "effective_neutral_mutations" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::effective_neutral_mutations(
                get_u(p, "total_mutations")?,
                get_f(p, "fraction_neutral")?,
            ),
        )),
        "nearly_neutral_boundary" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::nearly_neutral_boundary(get_f(p, "ne")?),
        )),
        "evolution_tajima_d" => Ok(RunOutput::Scalar(bio::evolution::neutral_theory::tajima_d(
            get_f(p, "pi")?,
            get_f(p, "theta_w")?,
            get_u(p, "n")?,
        ))),
        "evolution_watterson_theta" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::watterson_theta(
                get_u(p, "segregating_sites")?,
                get_u(p, "n")?,
                get_u(p, "sequence_length")?,
            ),
        )),
        "evolution_nucleotide_diversity" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::nucleotide_diversity(
                get_v(p, "differences")?,
                get_u(p, "n_sequences")?,
            ),
        )),
        "ewens_sampling_formula" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::ewens_sampling_formula(
                get_u(p, "n")?,
                get_f(p, "theta")?,
            ),
        )),
        "evolution_coalescent_expected_time" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::coalescent_expected_time(
                get_u(p, "n")?,
                get_f(p, "ne")?,
            ),
        )),
        "mcdonald_kreitman_ratio" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::mcdonald_kreitman_ratio(
                get_f(p, "dn")?,
                get_f(p, "ds")?,
                get_f(p, "pn")?,
                get_f(p, "ps")?,
            ),
        )),
        "neutrality_index" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::neutrality_index(
                get_f(p, "dn")?,
                get_f(p, "ds")?,
                get_f(p, "pn")?,
                get_f(p, "ps")?,
            ),
        )),
        "direction_of_selection" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::direction_of_selection(get_f(p, "ni")?),
        )),
        "speciation_rate_bdi" => Ok(RunOutput::Scalar(
            bio::evolution::speciation::speciation_rate_bdi(
                get_f(p, "lambda")?,
                get_f(p, "mu")?,
                get_f(p, "t")?,
                get_f(p, "n0")?,
            ),
        )),
        "allopatric_divergence" => Ok(RunOutput::Scalar(
            bio::evolution::speciation::allopatric_divergence(
                get_f(p, "d0")?,
                get_f(p, "mu")?,
                get_f(p, "t")?,
            ),
        )),
        "reproductive_isolation" => Ok(RunOutput::Scalar(
            bio::evolution::speciation::reproductive_isolation(
                get_f(p, "genetic_distance")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
            ),
        )),
        "reinforcement_strength" => Ok(RunOutput::Scalar(
            bio::evolution::speciation::reinforcement_strength(
                get_f(p, "sympatry")?,
                get_f(p, "hybrid_fitness")?,
            ),
        )),
        "yule_process_expected_species" => Ok(RunOutput::Scalar(
            bio::evolution::speciation::yule_process_expected_species(
                get_f(p, "lambda")?,
                get_f(p, "t")?,
            ),
        )),
        "birth_death_survival" => Ok(RunOutput::Scalar(
            bio::evolution::speciation::birth_death_survival(
                get_f(p, "lambda")?,
                get_f(p, "mu")?,
                get_f(p, "t")?,
            ),
        )),
        "character_displacement" => {
            let (a, b) = bio::evolution::speciation::character_displacement(
                get_f(p, "z1")?,
                get_f(p, "z2")?,
                get_f(p, "alpha")?,
                get_f(p, "sigma")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "ecological_speciation_fitness" => {
            let (a, b) = bio::evolution::speciation::ecological_speciation_fitness(
                get_f(p, "trait_val")?,
                get_f(p, "optimum1")?,
                get_f(p, "optimum2")?,
                get_f(p, "sigma")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "coalescent_expected_time" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::coalescent_expected_time(
                get_u(p, "n")?,
                get_f(p, "ne")?,
            ),
        )),
        "dn_ds_ratio" => Ok(RunOutput::Scalar(bio::evolution::molecular::dn_ds_ratio(
            get_f(p, "dn")?,
            get_f(p, "ds")?,
        ))),
        "frequency_dependent_fitness" => Ok(RunOutput::Scalar(
            bio::evolution::fitness::frequency_dependent_fitness(
                get_f(p, "freq")?,
                get_f(p, "advantage_rare")?,
            ),
        )),
        "molecular_clock_rate" => Ok(RunOutput::Scalar(
            bio::evolution::molecular::molecular_clock_rate(
                get_f(p, "substitutions")?,
                get_f(p, "divergence_time")?,
            ),
        )),
        "tajima_d" => Ok(RunOutput::Scalar(bio::evolution::neutral_theory::tajima_d(
            get_f(p, "pi")?,
            get_f(p, "theta_w")?,
            get_u(p, "n")?,
        ))),
        "nucleotide_diversity" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::nucleotide_diversity(
                get_v(p, "differences")?,
                get_u(p, "n_sequences")?,
            ),
        )),
        "watterson_theta" => Ok(RunOutput::Scalar(
            bio::evolution::neutral_theory::watterson_theta(
                get_u(p, "segregating_sites")?,
                get_u(p, "n")?,
                get_u(p, "sequence_length")?,
            ),
        )),
        "fitness_landscape_rugged" => {
            let gv = get_v(p, "genotype")?;
            let genotype: Vec<u8> = gv.iter().map(|&x| x as u8).collect();
            let pv = get_v(p, "peaks_flat")?;
            let pw = get_v(p, "peak_widths")?;
            let peak_len = genotype.len();
            let peaks: Vec<(Vec<u8>, f64)> = pv
                .chunks(peak_len)
                .zip(pw.iter())
                .map(|(g, &w)| (g.iter().map(|&x| x as u8).collect(), w))
                .collect();
            let peak_refs: Vec<(&[u8], f64)> =
                peaks.iter().map(|(g, w)| (g.as_slice(), *w)).collect();
            Ok(RunOutput::Scalar(
                bio::evolution::fitness::fitness_landscape_rugged(&genotype, &peak_refs),
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
