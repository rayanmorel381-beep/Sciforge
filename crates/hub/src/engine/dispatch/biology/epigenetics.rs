//! Dispatch handler for epigenetics functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "chromatin_accessibility" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::chromatin_accessibility(
                get_f(p, "open_fraction")?,
                get_f(p, "remodeler_activity")?,
                get_f(p, "histone_density")?,
            ),
        )),
        "nucleosome_occupancy" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::nucleosome_occupancy(
                get_f(p, "dna_affinity")?,
                get_f(p, "histone_conc")?,
                get_f(p, "competitor_conc")?,
            ),
        )),
        "histone_mark_propagation" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::histone_mark_propagation(
                get_f(p, "mark_fraction")?,
                get_f(p, "write_rate")?,
                get_f(p, "erase_rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "polycomb_silencing" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::polycomb_silencing(
                get_f(p, "pc_binding")?,
                get_f(p, "h3k27me3")?,
                get_f(p, "cooperative_factor")?,
            ),
        )),
        "trithorax_activation" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::trithorax_activation(
                get_f(p, "trx_binding")?,
                get_f(p, "h3k4me3")?,
                get_f(p, "cooperative_factor")?,
            ),
        )),
        "chromatin_loop_probability" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::chromatin_loop_probability(
                get_f(p, "distance_bp")?,
                get_f(p, "persistence_length_bp")?,
            ),
        )),
        "tad_insulation_score" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::tad_insulation_score(
                get_f(p, "contacts_within")?,
                get_f(p, "contacts_across")?,
            ),
        )),
        "enhancer_promoter_contact" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::enhancer_promoter_contact(
                get_f(p, "distance")?,
                get_f(p, "cohesin_loading")?,
                get_f(p, "ctcf_binding")?,
                get_f(p, "decay_length")?,
            ),
        )),
        "bivalent_resolution" => {
            let (a, b) = bio::epigenetics::chromatin::bivalent_resolution(
                get_f(p, "h3k4me3")?,
                get_f(p, "h3k27me3")?,
                get_f(p, "differentiation_signal")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "heterochromatin_spread" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::heterochromatin_spread(
                get_f(p, "hp1_conc")?,
                get_f(p, "h3k9me3")?,
                get_f(p, "barrier_strength")?,
                get_f(p, "distance")?,
            ),
        )),
        "atac_seq_signal" => Ok(RunOutput::Scalar(
            bio::epigenetics::chromatin::atac_seq_signal(
                get_f(p, "fragment_count")?,
                get_f(p, "total_reads")?,
                get_f(p, "region_length_bp")?,
            ),
        )),
        "histone_mark_occupancy" => Ok(RunOutput::Scalar(
            bio::epigenetics::histones::histone_mark_occupancy(
                get_f(p, "k_on")?,
                get_f(p, "k_off")?,
            ),
        )),
        "histone_mark_dynamics" => Ok(RunOutput::Scalar(
            bio::epigenetics::histones::histone_mark_dynamics(
                get_f(p, "occupancy")?,
                get_f(p, "k_on")?,
                get_f(p, "k_off")?,
                get_f(p, "k_spread")?,
                get_f(p, "neighbor_occ")?,
            ),
        )),
        "nucleosome_positioning_energy" => Ok(RunOutput::Scalar(
            bio::epigenetics::histones::nucleosome_positioning_energy(
                get_v(p, "dna_flexibility")?,
                get_u(p, "position")?,
                get_u(p, "wrap_length")?,
            ),
        )),
        "chromatin_compaction_ratio" => Ok(RunOutput::Scalar(
            bio::epigenetics::histones::chromatin_compaction_ratio(
                get_f(p, "extended_length")?,
                get_f(p, "compacted_length")?,
            ),
        )),
        "histone_acetylation_equilibrium" => Ok(RunOutput::Scalar(
            bio::epigenetics::histones::histone_acetylation_equilibrium(
                get_f(p, "hat_activity")?,
                get_f(p, "hdac_activity")?,
            ),
        )),
        "bivalent_domain_resolution" => {
            let (a, b) = bio::epigenetics::histones::bivalent_domain_resolution(
                get_f(p, "h3k4me3")?,
                get_f(p, "h3k27me3")?,
                get_f(p, "signal")?,
                get_f(p, "threshold")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "chip_seq_enrichment" => Ok(RunOutput::Scalar(
            bio::epigenetics::histones::chip_seq_enrichment(
                get_f(p, "ip_reads")?,
                get_f(p, "input_reads")?,
                get_f(p, "ip_total")?,
                get_f(p, "input_total")?,
            ),
        )),
        "reader_writer_feedback" => Ok(RunOutput::Scalar(
            bio::epigenetics::histones::reader_writer_feedback(
                get_f(p, "mark")?,
                get_f(p, "reader_affinity")?,
                get_f(p, "writer_rate")?,
                get_f(p, "eraser_rate")?,
            ),
        )),
        "epigenetic_inheritance_probability" => Ok(RunOutput::Scalar(
            bio::epigenetics::inheritance::epigenetic_inheritance_probability(
                get_f(p, "maintenance")?,
                get_u(p, "generations")?,
            ),
        )),
        "transgenerational_decay" => Ok(RunOutput::Vector(
            bio::epigenetics::inheritance::transgenerational_decay(
                get_f(p, "mark")?,
                get_f(p, "reset_rate")?,
                get_u(p, "generations")?,
            ),
        )),
        "epimutation_rate" => Ok(RunOutput::Scalar(
            bio::epigenetics::inheritance::epimutation_rate(
                get_u(p, "changes")?,
                get_u(p, "sites")?,
                get_u(p, "generations")?,
            ),
        )),
        "epiallele_frequency" => Ok(RunOutput::Vector(
            bio::epigenetics::inheritance::epiallele_frequency(
                get_f(p, "fitness_epi")?,
                get_f(p, "fitness_normal")?,
                get_f(p, "freq")?,
                get_u(p, "generations")?,
            ),
        )),
        "chromatin_state_transition" => Ok(RunOutput::Vector(
            bio::epigenetics::inheritance::chromatin_state_transition(
                get_v(p, "state")?,
                get_m(p, "transition_matrix")?,
            ),
        )),
        "imprinting_expression" => Ok(RunOutput::Scalar(
            bio::epigenetics::inheritance::imprinting_expression(
                get_f(p, "maternal")?,
                get_f(p, "paternal")?,
                get_b(p, "imprint_maternal")?,
            ),
        )),
        "paramutation" => Ok(RunOutput::PairVec(
            bio::epigenetics::inheritance::paramutation(
                get_f(p, "strong_allele")?,
                get_f(p, "weak_allele")?,
                get_f(p, "conversion_rate")?,
                get_u(p, "generations")?,
            ),
        )),
        "metastable_epiallele" => Ok(RunOutput::Scalar(
            bio::epigenetics::inheritance::metastable_epiallele(
                get_f(p, "base_expression")?,
                get_f(p, "methylation")?,
                get_f(p, "stochastic_variance")?,
            ),
        )),
        "epigenetic_clock" => Ok(RunOutput::Scalar(
            bio::epigenetics::inheritance::epigenetic_clock(
                get_v(p, "methylation_sites")?,
                get_v(p, "coefficients")?,
                get_f(p, "intercept")?,
            ),
        )),
        "environmental_epigenetic_response" => Ok(RunOutput::Scalar(
            bio::epigenetics::inheritance::environmental_epigenetic_response(
                get_f(p, "stress")?,
                get_f(p, "sensitivity")?,
                get_f(p, "methylation_change_rate")?,
                get_f(p, "baseline_methylation")?,
            ),
        )),
        "cpg_methylation_level" => Ok(RunOutput::Scalar(
            bio::epigenetics::methylation::cpg_methylation_level(
                get_u(p, "methylated")?,
                get_u(p, "total_cpg")?,
            ),
        )),
        "methylation_decay" => Ok(RunOutput::Vector(
            bio::epigenetics::methylation::methylation_decay(
                get_f(p, "level")?,
                get_f(p, "dilution_rate")?,
                get_f(p, "maintenance_efficiency")?,
                get_u(p, "generations")?,
            ),
        )),
        "de_novo_methylation" => Ok(RunOutput::Scalar(
            bio::epigenetics::methylation::de_novo_methylation(
                get_f(p, "unmethylated")?,
                get_f(p, "rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "bisulfite_conversion_efficiency" => Ok(RunOutput::Scalar(
            bio::epigenetics::methylation::bisulfite_conversion_efficiency(
                get_u(p, "converted_c")?,
                get_u(p, "total_c")?,
            ),
        )),
        "methylation_entropy" => Ok(RunOutput::Scalar(
            bio::epigenetics::methylation::methylation_entropy(get_v(p, "profile")?),
        )),
        "hydroxymethylation_rate" => Ok(RunOutput::Scalar(
            bio::epigenetics::methylation::hydroxymethylation_rate(
                get_f(p, "methylation")?,
                get_f(p, "tet_activity")?,
            ),
        )),
        "tet_oxidation_cascade" => {
            let (a, b, c) = bio::epigenetics::methylation::tet_oxidation_cascade(
                get_f(p, "mc")?,
                get_f(p, "tet")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "dnmt_processivity" => Ok(RunOutput::Vector(
            bio::epigenetics::methylation::dnmt_processivity(
                get_v(p, "initial_methylation")?,
                get_f(p, "processivity")?,
                get_b(p, "direction_forward")?,
            ),
        )),
        "cpg_island_density" => Ok(RunOutput::Scalar(
            bio::epigenetics::methylation::cpg_island_density(
                get_u(p, "sequence_length")?,
                get_u(p, "cpg_count")?,
                get_f(p, "expected_cpg")?,
            ),
        )),
        "age_methylation_predictor" => Ok(RunOutput::Scalar(
            bio::epigenetics::methylation::age_methylation_predictor(
                get_v(p, "cpg_beta_values")?,
                get_v(p, "weights")?,
                get_f(p, "offset")?,
            ),
        )),
        "mirna_target_repression" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::mirna_target_repression(
                get_f(p, "mirna_conc")?,
                get_f(p, "target_mrna")?,
                get_f(p, "kd")?,
                get_f(p, "max_repression")?,
            ),
        )),
        "mirna_seed_match_score" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::mirna_seed_match_score(
                get_u(p, "seed_matches")?,
                get_f(p, "three_prime_pairing")?,
                get_f(p, "site_accessibility")?,
            ),
        )),
        "lncrna_scaffold_activity" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::lncrna_scaffold_activity(
                get_f(p, "lncrna_conc")?,
                get_f(p, "protein_a")?,
                get_f(p, "protein_b")?,
                get_f(p, "kd_a")?,
                get_f(p, "kd_b")?,
            ),
        )),
        "xist_silencing_spread" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::xist_silencing_spread(
                get_f(p, "xist_expression")?,
                get_f(p, "distance_from_xic")?,
                get_f(p, "spread_rate")?,
            ),
        )),
        "pirna_transposon_silencing" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::pirna_transposon_silencing(
                get_f(p, "pirna_conc")?,
                get_f(p, "transposon_copies")?,
                get_f(p, "silencing_efficiency")?,
            ),
        )),
        "circular_rna_mirna_sponge" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::circular_rna_mirna_sponge(
                get_f(p, "circrna")?,
                get_f(p, "binding_sites")?,
                get_f(p, "mirna_total")?,
                get_f(p, "kd")?,
            ),
        )),
        "rnai_knockdown_efficiency" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::rnai_knockdown_efficiency(
                get_f(p, "sirna_conc")?,
                get_f(p, "target_mrna")?,
                get_f(p, "risc_activity")?,
                get_f(p, "kd")?,
            ),
        )),
        "enhancer_rna_activity" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::enhancer_rna_activity(
                get_f(p, "erna_level")?,
                get_f(p, "enhancer_activity_base")?,
                get_f(p, "amplification")?,
            ),
        )),
        "antisense_rna_regulation" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::antisense_rna_regulation(
                get_f(p, "sense_mrna")?,
                get_f(p, "antisense_rna")?,
                get_f(p, "duplex_rate")?,
                get_f(p, "degradation_rate")?,
            ),
        )),
        "ncrna_mediated_methylation" => Ok(RunOutput::Scalar(
            bio::epigenetics::noncoding_rna::ncrna_mediated_methylation(
                get_f(p, "ncrna_guide")?,
                get_f(p, "target_region_accessibility")?,
                get_f(p, "dnmt_activity")?,
            ),
        )),
        "heterochromatin_spreading" => {
            let mut marks = get_v(p, "marks")?.to_vec();
            let bp = get_v(p, "barrier_positions")?;
            let barriers: Vec<usize> = bp.iter().map(|&x| x as usize).collect();
            bio::epigenetics::histones::heterochromatin_spreading(
                &mut marks,
                get_f(p, "spread_rate")?,
                &barriers,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Vector(marks))
        }
        "histone_spread_simulate" => {
            let mut occ = get_v(p, "occupancies")?.to_vec();
            let result = bio::epigenetics::histones::histone_spread_simulate(
                &mut occ,
                get_f(p, "k_on")?,
                get_f(p, "k_off")?,
                get_f(p, "k_spread")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            let flat: Vec<f64> = result.into_iter().flatten().collect();
            Ok(RunOutput::Vector(flat))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
