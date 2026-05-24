//! Dispatch handler for synthetic biology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "synbio_toggle_switch" => {
            let (a, b) = bio::synthetic_biology::circuits::toggle_switch(
                get_f(p, "u")?,
                get_f(p, "v")?,
                get_f(p, "alpha1")?,
                get_f(p, "alpha2")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "n")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "synbio_repressilator" => {
            let x_v = get_v(p, "x")?;
            let x = [x_v[0], x_v[1], x_v[2]];
            {
                let r = bio::synthetic_biology::circuits::repressilator(
                    &x,
                    get_f(p, "alpha")?,
                    get_f(p, "alpha0")?,
                    get_f(p, "beta")?,
                    get_f(p, "n")?,
                );
                Ok(RunOutput::Triple(r[0], r[1], r[2]))
            }
        }
        "hill_activation" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::hill_activation(
                get_f(p, "x")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
            ),
        )),
        "hill_repression" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::hill_repression(
                get_f(p, "x")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
            ),
        )),
        "inducible_promoter" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::inducible_promoter(
                get_f(p, "inducer")?,
                get_f(p, "kd")?,
                get_f(p, "n")?,
                get_f(p, "basal")?,
                get_f(p, "max_rate")?,
            ),
        )),
        "ribosome_binding_strength" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::ribosome_binding_strength(
                get_f(p, "rbs_score")?,
                get_f(p, "max_translation")?,
            ),
        )),
        "and_gate" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::and_gate(
                get_f(p, "input_a")?,
                get_f(p, "input_b")?,
                get_f(p, "k_a")?,
                get_f(p, "k_b")?,
                get_f(p, "n")?,
                get_f(p, "max_output")?,
            ),
        )),
        "or_gate" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::or_gate(
                get_f(p, "input_a")?,
                get_f(p, "input_b")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
                get_f(p, "max_output")?,
            ),
        )),
        "not_gate" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::not_gate(
                get_f(p, "input")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
                get_f(p, "max_output")?,
            ),
        )),
        "oscillator_amplitude" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::oscillator_amplitude(get_v(p, "protein_levels")?),
        )),
        "nand_gate" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::nand_gate(
                get_f(p, "input_a")?,
                get_f(p, "input_b")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
                get_f(p, "max_output")?,
            ),
        )),
        "xor_gate" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::xor_gate(
                get_f(p, "input_a")?,
                get_f(p, "input_b")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
                get_f(p, "max_output")?,
            ),
        )),
        "feed_forward_loop_c1" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::feed_forward_loop_c1(
                get_f(p, "input")?,
                get_f(p, "x")?,
                get_f(p, "k_input")?,
                get_f(p, "k_x")?,
                get_f(p, "n")?,
                get_f(p, "max_rate")?,
            ),
        )),
        "negative_autoregulation" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::negative_autoregulation(
                get_f(p, "protein")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
                get_f(p, "basal")?,
                get_f(p, "max_rate")?,
            ),
        )),
        "positive_autoregulation" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::positive_autoregulation(
                get_f(p, "protein")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
                get_f(p, "basal")?,
                get_f(p, "max_rate")?,
            ),
        )),
        "bandpass_filter" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::bandpass_filter(
                get_f(p, "input")?,
                get_f(p, "k_low")?,
                get_f(p, "k_high")?,
                get_f(p, "n")?,
                get_f(p, "max_output")?,
            ),
        )),
        "quorum_sensing_autoinducer" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::quorum_sensing_autoinducer(
                get_f(p, "cell_density")?,
                get_f(p, "production_rate")?,
                get_f(p, "degradation_rate")?,
            ),
        )),
        "quorum_sensing_response" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::quorum_sensing_response(
                get_f(p, "autoinducer")?,
                get_f(p, "threshold")?,
                get_f(p, "n")?,
                get_f(p, "max_response")?,
            ),
        )),
        "kill_switch_toxin_antitoxin" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::kill_switch_toxin_antitoxin(
                get_f(p, "inducer")?,
                get_f(p, "k_toxin")?,
                get_f(p, "k_antitoxin")?,
                get_f(p, "n")?,
            ),
        )),
        "metabolic_burden" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::metabolic_burden(
                get_f(p, "circuit_protein")?,
                get_f(p, "growth_rate_max")?,
                get_f(p, "burden_coeff")?,
            ),
        )),
        "biosensor_dose_response" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::circuits::biosensor_dose_response(
                get_f(p, "analyte")?,
                get_f(p, "k_half")?,
                get_f(p, "n")?,
                get_f(p, "output_min")?,
                get_f(p, "output_max")?,
            ),
        )),
        "crispr_on_target_score" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::crispr::crispr_on_target_score(
                get_f(p, "gc_content")?,
                get_v(p, "position_scores")?,
            ),
        )),
        "cas9_cleavage_efficiency" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::crispr::cas9_cleavage_efficiency(
                get_f(p, "grna_binding")?,
                get_f(p, "pam_strength")?,
                get_f(p, "chromatin_accessibility")?,
            ),
        )),
        "hdr_efficiency" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::crispr::hdr_efficiency(
                get_f(p, "template_conc")?,
                get_f(p, "homology_arm_length")?,
                get_f(p, "cell_cycle_s_fraction")?,
            ),
        )),
        "nhej_indel_spectrum" => Ok(RunOutput::Vector(
            bio::synthetic_biology::crispr::nhej_indel_spectrum(
                get_f(p, "cut_position")?,
                get_v(p, "microhomology_scores")?,
            ),
        )),
        "base_editing_efficiency" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::crispr::base_editing_efficiency(
                get_b(p, "c_to_t_window")?,
                get_f(p, "accessibility")?,
                get_f(p, "deaminase_activity")?,
            ),
        )),
        "prime_editing_efficiency" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::crispr::prime_editing_efficiency(
                get_u(p, "pbs_length")?,
                get_u(p, "rt_template_length")?,
                get_u(p, "nick_distance")?,
            ),
        )),
        "guide_rna_folding_penalty" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::crispr::guide_rna_folding_penalty(get_f(
                p,
                "self_complementarity_score",
            )?),
        )),
        "multiplex_editing_probability" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::crispr::multiplex_editing_probability(get_v(
                p,
                "efficiencies",
            )?),
        )),
        "gene_drive_frequency" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::crispr::gene_drive_frequency(
                get_f(p, "drive_efficiency")?,
                get_f(p, "fitness_cost")?,
                get_u(p, "generations")?,
                get_f(p, "initial_freq")?,
            ),
        )),
        "flux_balance_objective" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::metabolic_engineering::flux_balance_objective(
                get_v(p, "fluxes")?,
                get_v(p, "objective_coefficients")?,
            ),
        )),
        "theoretical_yield" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::metabolic_engineering::theoretical_yield(
                get_u(p, "substrate_carbons")?,
                get_u(p, "product_carbons")?,
                get_f(p, "pathway_efficiency")?,
            ),
        )),
        "heterologous_metabolic_burden" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::metabolic_engineering::heterologous_metabolic_burden(
                get_f(p, "heterologous_protein_fraction")?,
                get_f(p, "growth_rate_wt")?,
            ),
        )),
        "pathway_thermodynamic_driving_force" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::metabolic_engineering::pathway_thermodynamic_driving_force(
                get_v(p, "delta_g_steps")?,
            ),
        )),
        "enzyme_cost_minimization" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::metabolic_engineering::enzyme_cost_minimization(
                get_v(p, "fluxes")?,
                get_v(p, "kcats")?,
            ),
        )),
        "cofactor_balance" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::metabolic_engineering::cofactor_balance(
                get_f(p, "nadh_produced")?,
                get_f(p, "nadh_consumed")?,
            ),
        )),
        "titer_rate_yield" => {
            let (a, b) = bio::synthetic_biology::metabolic_engineering::titer_rate_yield(
                get_f(p, "titer")?,
                get_f(p, "time")?,
                get_f(p, "substrate_consumed")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "gene_expression_burden" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::metabolic_engineering::gene_expression_burden(
                get_f(p, "promoter_strength")?,
                get_f(p, "copy_number")?,
                get_f(p, "protein_size")?,
            ),
        )),
        "heterologous_pathway_flux" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::metabolic_engineering::heterologous_pathway_flux(
                get_v(p, "enzyme_levels")?,
                get_v(p, "kms")?,
                get_v(p, "substrates")?,
            ),
        )),
        "stochastic_gene_expression_mean" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::stochastic_gene_expression_mean(
                get_f(p, "transcription_rate")?,
                get_f(p, "translation_rate")?,
                get_f(p, "mrna_decay")?,
                get_f(p, "protein_decay")?,
            ),
        )),
        "stochastic_gene_expression_variance" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::stochastic_gene_expression_variance(
                get_f(p, "transcription_rate")?,
                get_f(p, "translation_rate")?,
                get_f(p, "mrna_decay")?,
                get_f(p, "protein_decay")?,
            ),
        )),
        "synbio_fano_factor" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::fano_factor(
                get_f(p, "variance")?,
                get_f(p, "mean")?,
            ),
        )),
        "noise_intrinsic" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::noise_intrinsic(
                get_f(p, "burst_size")?,
                get_f(p, "mean_protein")?,
            ),
        )),
        "noise_extrinsic" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::noise_extrinsic(get_f(p, "cv_parameter")?),
        )),
        "total_noise" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::total_noise(
                get_f(p, "intrinsic")?,
                get_f(p, "extrinsic")?,
            ),
        )),
        "gillespie_propensity_birth" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::gillespie_propensity_birth(get_f(p, "rate")?),
        )),
        "gillespie_propensity_death" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::gillespie_propensity_death(
                get_f(p, "rate")?,
                get_f(p, "population")?,
            ),
        )),
        "gillespie_tau" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::gillespie_tau(
                get_f(p, "total_propensity")?,
                get_f(p, "random_uniform")?,
            ),
        )),
        "burst_frequency" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::burst_frequency(
                get_f(p, "transcription_rate")?,
                get_f(p, "promoter_off_rate")?,
            ),
        )),
        "burst_size_mean" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::burst_size_mean(
                get_f(p, "translation_rate")?,
                get_f(p, "mrna_decay")?,
            ),
        )),
        "coefficient_of_variation_squared" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::coefficient_of_variation_squared(
                get_f(p, "mean")?,
                get_f(p, "variance")?,
            ),
        )),
        "two_state_promoter_mean" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::two_state_promoter_mean(
                get_f(p, "k_on")?,
                get_f(p, "k_off")?,
                get_f(p, "transcription")?,
                get_f(p, "mrna_decay")?,
            ),
        )),
        "two_state_promoter_fano" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::two_state_promoter_fano(
                get_f(p, "k_on")?,
                get_f(p, "k_off")?,
                get_f(p, "transcription")?,
                get_f(p, "mrna_decay")?,
            ),
        )),
        "langevin_approximation_step" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::langevin_approximation_step(
                get_f(p, "x")?,
                get_f(p, "drift")?,
                get_f(p, "diffusion")?,
                get_f(p, "dt")?,
                get_f(p, "noise")?,
            ),
        )),
        "chemical_master_equation_steady_state_poisson" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::chemical_master_equation_steady_state_poisson(
                get_f(p, "production")?,
                get_f(p, "degradation")?,
            ),
        )),
        "binomial_partitioning_noise" => Ok(RunOutput::Scalar(
            bio::synthetic_biology::stochastic::binomial_partitioning_noise(
                get_f(p, "n")?,
                get_f(p, "p")?,
            ),
        )),
        "gene_expression_delay_gamma" => {
            let shape = get_i(p, "shape")? as u32;
            Ok(RunOutput::Scalar(
                bio::synthetic_biology::stochastic::gene_expression_delay_gamma(
                    get_f(p, "mean_delay")?,
                    shape,
                    get_f(p, "t")?,
                ),
            ))
        }
        "crispr_off_target_cfd" => {
            let v = get_v(p, "mismatches")?;
            let mis: Vec<(usize, f64)> = v.chunks(2).map(|c| (c[0] as usize, c[1])).collect();
            Ok(RunOutput::Scalar(
                bio::synthetic_biology::crispr::crispr_off_target_cfd(&mis),
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
