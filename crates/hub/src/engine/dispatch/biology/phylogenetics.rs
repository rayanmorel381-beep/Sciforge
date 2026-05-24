//! Dispatch handler for phylogenetics functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "aa_to_index" => {
            let aa = get_i(p, "aa")? as u8;
            Ok(RunOutput::Integer(
                bio::phylogenetics::alignment::aa_to_index(aa)
                    .map(|x| x as i64)
                    .unwrap_or(-1),
            ))
        }
        "blosum62_score" => {
            let a = get_i(p, "a")? as u8;
            let b = get_i(p, "b")? as u8;
            Ok(RunOutput::Integer(
                bio::phylogenetics::alignment::blosum62_score(a, b) as i64,
            ))
        }
        "needleman_wunsch" => {
            let seq_a = get_str(p, "seq_a")?.as_bytes();
            let seq_b = get_str(p, "seq_b")?.as_bytes();
            let gap_penalty = get_i(p, "gap_penalty")? as i32;
            {
                let (a, b, s) =
                    bio::phylogenetics::alignment::needleman_wunsch(seq_a, seq_b, gap_penalty);
                Ok(RunOutput::Text(format!(
                    "{},{},{}",
                    String::from_utf8_lossy(&a),
                    String::from_utf8_lossy(&b),
                    s
                )))
            }
        }
        "smith_waterman" => {
            let seq_a = get_str(p, "seq_a")?.as_bytes();
            let seq_b = get_str(p, "seq_b")?.as_bytes();
            let gap_penalty = get_i(p, "gap_penalty")? as i32;
            {
                let (a, b, s) =
                    bio::phylogenetics::alignment::smith_waterman(seq_a, seq_b, gap_penalty);
                Ok(RunOutput::Text(format!(
                    "{},{},{}",
                    String::from_utf8_lossy(&a),
                    String::from_utf8_lossy(&b),
                    s
                )))
            }
        }
        "alignment_identity" => {
            let align_a = get_str(p, "align_a")?.as_bytes();
            let align_b = get_str(p, "align_b")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::phylogenetics::alignment::alignment_identity(align_a, align_b),
            ))
        }
        "affine_gap_needleman_wunsch" => {
            let seq_a = get_str(p, "seq_a")?.as_bytes();
            let seq_b = get_str(p, "seq_b")?.as_bytes();
            let gap_open = get_i(p, "gap_open")? as i32;
            let gap_extend = get_i(p, "gap_extend")? as i32;
            {
                let (a, b, s) = bio::phylogenetics::alignment::affine_gap_needleman_wunsch(
                    seq_a, seq_b, gap_open, gap_extend,
                );
                Ok(RunOutput::Text(format!(
                    "{},{},{}",
                    String::from_utf8_lossy(&a),
                    String::from_utf8_lossy(&b),
                    s
                )))
            }
        }
        "multiple_alignment_score" => {
            let alignment_strs = get_str(p, "alignment")?;
            let alignment_vecs: Vec<Vec<u8>> = alignment_strs
                .split(',')
                .map(|s| s.as_bytes().to_vec())
                .collect();
            Ok(RunOutput::Integer(
                bio::phylogenetics::alignment::multiple_alignment_score(&alignment_vecs) as i64,
            ))
        }
        "pairwise_distance" => {
            let align_a = get_str(p, "align_a")?.as_bytes();
            let align_b = get_str(p, "align_b")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::phylogenetics::alignment::pairwise_distance(align_a, align_b),
            ))
        }
        "jukes_cantor_distance" => Ok(RunOutput::Scalar(
            bio::phylogenetics::alignment::jukes_cantor_distance(get_f(p, "p_distance")?),
        )),
        "gap_fraction" => {
            let aligned = get_str(p, "aligned")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::phylogenetics::alignment::gap_fraction(aligned),
            ))
        }
        "phylo_hamming_distance" => {
            let a = get_str(p, "a")?.as_bytes();
            let b = get_str(p, "b")?.as_bytes();
            Ok(RunOutput::Integer(
                bio::phylogenetics::distance::hamming_distance(a, b) as i64,
            ))
        }
        "p_distance" => {
            let a = get_str(p, "a")?.as_bytes();
            let b = get_str(p, "b")?.as_bytes();
            Ok(RunOutput::Scalar(bio::phylogenetics::distance::p_distance(
                a, b,
            )))
        }
        "jukes_cantor" => Ok(RunOutput::Scalar(
            bio::phylogenetics::distance::jukes_cantor(get_f(p, "p")?),
        )),
        "kimura_2p" => Ok(RunOutput::Scalar(bio::phylogenetics::distance::kimura_2p(
            get_f(p, "transitions")?,
            get_f(p, "transversions")?,
            get_f(p, "length")?,
        ))),
        "count_transitions_transversions" => {
            let a = get_str(p, "a")?.as_bytes();
            let b = get_str(p, "b")?.as_bytes();
            {
                let (a, b) = bio::phylogenetics::distance::count_transitions_transversions(a, b);
                Ok(RunOutput::Pair(a as f64, b as f64))
            }
        }
        "distance_matrix" => {
            let sequences_strs = get_str(p, "sequences")?;
            let sequences_parts: Vec<&[u8]> =
                sequences_strs.split(',').map(|s| s.as_bytes()).collect();
            Ok(RunOutput::Matrix(
                bio::phylogenetics::distance::distance_matrix(&sequences_parts),
            ))
        }
        "log_det_distance" => {
            let freq_matrix_m = get_m(p, "freq_matrix")?;
            let freq_matrix = [
                [
                    freq_matrix_m[0][0],
                    freq_matrix_m[0][1],
                    freq_matrix_m[0][2],
                    freq_matrix_m[0][3],
                ],
                [
                    freq_matrix_m[1][0],
                    freq_matrix_m[1][1],
                    freq_matrix_m[1][2],
                    freq_matrix_m[1][3],
                ],
                [
                    freq_matrix_m[2][0],
                    freq_matrix_m[2][1],
                    freq_matrix_m[2][2],
                    freq_matrix_m[2][3],
                ],
                [
                    freq_matrix_m[3][0],
                    freq_matrix_m[3][1],
                    freq_matrix_m[3][2],
                    freq_matrix_m[3][3],
                ],
            ];
            Ok(RunOutput::Scalar(
                bio::phylogenetics::distance::log_det_distance(&freq_matrix),
            ))
        }
        "phylo_molecular_clock_rate" => Ok(RunOutput::Scalar(
            bio::phylogenetics::molecular_clock::molecular_clock_rate(
                get_f(p, "substitutions")?,
                get_f(p, "divergence_time")?,
            ),
        )),
        "strict_clock_branch_length" => Ok(RunOutput::Scalar(
            bio::phylogenetics::molecular_clock::strict_clock_branch_length(
                get_f(p, "rate")?,
                get_f(p, "time")?,
            ),
        )),
        "relaxed_clock_lognormal" => Ok(RunOutput::Scalar(
            bio::phylogenetics::molecular_clock::relaxed_clock_lognormal(
                get_f(p, "mean_rate")?,
                get_f(p, "sigma")?,
                get_f(p, "branch_deviation")?,
            ),
        )),
        "divergence_time_from_distance" => Ok(RunOutput::Scalar(
            bio::phylogenetics::molecular_clock::divergence_time_from_distance(
                get_f(p, "genetic_distance")?,
                get_f(p, "substitution_rate")?,
            ),
        )),
        "jc_distance" => Ok(RunOutput::Scalar(
            bio::phylogenetics::molecular_clock::jc_distance(get_f(p, "p_diff")?),
        )),
        "kimura_2p_distance" => Ok(RunOutput::Scalar(
            bio::phylogenetics::molecular_clock::kimura_2p_distance(
                get_f(p, "transitions")?,
                get_f(p, "transversions")?,
                get_f(p, "length")?,
            ),
        )),
        "bayesian_clock_calibration" => {
            let (a, b) = bio::phylogenetics::molecular_clock::bayesian_clock_calibration(
                get_f(p, "prior_age")?,
                get_f(p, "prior_sd")?,
                get_f(p, "likelihood_age")?,
                get_f(p, "likelihood_sd")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "rate_heterogeneity_gamma" => Ok(RunOutput::Vector(
            bio::phylogenetics::molecular_clock::rate_heterogeneity_gamma(
                get_f(p, "alpha")?,
                get_u(p, "n_categories")?,
            ),
        )),
        "local_clock_assignment" => Ok(RunOutput::IntVector(
            bio::phylogenetics::molecular_clock::local_clock_assignment(
                get_v(p, "branch_rates")?,
                get_f(p, "threshold")?,
            )
            .into_iter()
            .map(|x| x as i64)
            .collect(),
        )),
        "phylo_gc_content" => {
            let seq = get_str(p, "seq")?.as_bytes();
            Ok(RunOutput::Scalar(bio::phylogenetics::sequence::gc_content(
                seq,
            )))
        }
        "complement" => {
            let base = get_i(p, "base")? as u8;
            Ok(RunOutput::Integer(
                bio::phylogenetics::sequence::complement(base) as i64,
            ))
        }
        "phylo_reverse_complement" => {
            let seq = get_str(p, "seq")?.as_bytes();
            Ok(RunOutput::Text(
                String::from_utf8_lossy(&bio::phylogenetics::sequence::reverse_complement(seq))
                    .into_owned(),
            ))
        }
        "translate_codon" => {
            let codon = get_str(p, "codon")?.as_bytes();
            Ok(RunOutput::Integer(
                bio::phylogenetics::sequence::translate_codon(codon) as i64,
            ))
        }
        "phylo_translate" => {
            let dna = get_str(p, "dna")?.as_bytes();
            Ok(RunOutput::Text(
                String::from_utf8_lossy(&bio::phylogenetics::sequence::translate(dna)).into_owned(),
            ))
        }
        "transcribe" => {
            let dna = get_str(p, "dna")?.as_bytes();
            Ok(RunOutput::Text(
                String::from_utf8_lossy(&bio::phylogenetics::sequence::transcribe(dna))
                    .into_owned(),
            ))
        }
        "nucleotide_frequencies" => {
            let seq = get_str(p, "seq")?.as_bytes();
            {
                let r = bio::phylogenetics::sequence::nucleotide_frequencies(seq);
                Ok(RunOutput::Vector(r.to_vec()))
            }
        }
        "molecular_weight_dna" => {
            let seq = get_str(p, "seq")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::phylogenetics::sequence::molecular_weight_dna(seq),
            ))
        }
        "phylo_melting_temperature_basic" => {
            let seq = get_str(p, "seq")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::phylogenetics::sequence::melting_temperature_basic(seq),
            ))
        }
        "upgma" => Ok(RunOutput::Matrix(
            bio::phylogenetics::tree::upgma(get_m(p, "dist_matrix")?)
                .into_iter()
                .map(|(a, b, c)| vec![a as f64, b as f64, c])
                .collect(),
        )),
        "neighbor_joining" => Ok(RunOutput::Matrix(
            bio::phylogenetics::tree::neighbor_joining(get_m(p, "dist_matrix")?)
                .into_iter()
                .map(|(a, b, c, d)| vec![a as f64, b as f64, c, d])
                .collect(),
        )),
        "wpgma" => Ok(RunOutput::Matrix(
            bio::phylogenetics::tree::wpgma(get_m(p, "dist_matrix")?)
                .into_iter()
                .map(|(a, b, c)| vec![a as f64, b as f64, c])
                .collect(),
        )),
        "molecular_clock_test" => Ok(RunOutput::Scalar(
            bio::phylogenetics::tree::molecular_clock_test(
                get_v(p, "branch_lengths")?,
                get_v(p, "expected")?,
            ),
        )),
        "robinson_foulds" => {
            let splits_a_m = get_m(p, "splits_a")?;
            let splits_a: Vec<Vec<bool>> = splits_a_m
                .iter()
                .map(|r| r.iter().map(|&x| x != 0.0).collect())
                .collect();
            let splits_b_m = get_m(p, "splits_b")?;
            let splits_b: Vec<Vec<bool>> = splits_b_m
                .iter()
                .map(|r| r.iter().map(|&x| x != 0.0).collect())
                .collect();
            Ok(RunOutput::Integer(
                bio::phylogenetics::tree::robinson_foulds(&splits_a, &splits_b) as i64,
            ))
        }
        "sackin_index" => {
            let branch_depths_v = get_v(p, "branch_depths")?;
            let branch_depths: Vec<usize> = branch_depths_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(bio::phylogenetics::tree::sackin_index(
                &branch_depths,
            )))
        }
        "colless_index" => {
            let left_sizes_v = get_v(p, "left_sizes")?;
            let left_sizes: Vec<usize> = left_sizes_v.iter().map(|&x| x as usize).collect();
            let right_sizes_v = get_v(p, "right_sizes")?;
            let right_sizes: Vec<usize> = right_sizes_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(bio::phylogenetics::tree::colless_index(
                &left_sizes,
                &right_sizes,
            )))
        }
        "branch_length_total" => Ok(RunOutput::Scalar(
            bio::phylogenetics::tree::branch_length_total(get_v(p, "branch_lengths")?),
        )),
        "patristic_distance" => Ok(RunOutput::Scalar(
            bio::phylogenetics::tree::patristic_distance(
                get_m(p, "tree_distances")?,
                get_u(p, "i")?,
                get_u(p, "j")?,
            ),
        )),
        "gamma_rate_categories" => Ok(RunOutput::Vector(
            bio::phylogenetics::tree::gamma_rate_categories(
                get_f(p, "alpha")?,
                get_u(p, "n_categories")?,
            ),
        )),
        "parsimony_score" => {
            let sv = get_v(p, "sequences_flat")?;
            let seq_len = get_u(p, "seq_len")?;
            let seqs_u8: Vec<Vec<u8>> = sv
                .chunks(seq_len)
                .map(|c| c.iter().map(|&x| x as u8).collect())
                .collect();
            let seq_refs: Vec<&[u8]> = seqs_u8.iter().map(|s| s.as_slice()).collect();
            let tv = get_v(p, "tree")?;
            let tree: Vec<(usize, usize)> = tv
                .chunks(2)
                .map(|c| (c[0] as usize, c[1] as usize))
                .collect();
            Ok(RunOutput::Integer(
                bio::phylogenetics::tree::parsimony_score(&seq_refs, &tree) as i64,
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
