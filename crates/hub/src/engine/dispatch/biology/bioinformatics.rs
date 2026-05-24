//! Dispatch handler for bioinformatics functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "smith_waterman_score" => {
            let seq1 = get_str(p, "seq1")?.as_bytes();
            let seq2 = get_str(p, "seq2")?.as_bytes();
            let match_score = get_i(p, "match_score")? as i32;
            let mismatch = get_i(p, "mismatch")? as i32;
            let gap = get_i(p, "gap")? as i32;
            Ok(RunOutput::Integer(
                bio::bioinformatics::alignment::smith_waterman_score(
                    seq1,
                    seq2,
                    match_score,
                    mismatch,
                    gap,
                ) as i64,
            ))
        }
        "needleman_wunsch_score" => {
            let seq1 = get_str(p, "seq1")?.as_bytes();
            let seq2 = get_str(p, "seq2")?.as_bytes();
            let match_score = get_i(p, "match_score")? as i32;
            let mismatch = get_i(p, "mismatch")? as i32;
            let gap = get_i(p, "gap")? as i32;
            Ok(RunOutput::Integer(
                bio::bioinformatics::alignment::needleman_wunsch_score(
                    seq1,
                    seq2,
                    match_score,
                    mismatch,
                    gap,
                ) as i64,
            ))
        }
        "edit_distance" => {
            let seq1 = get_str(p, "seq1")?.as_bytes();
            let seq2 = get_str(p, "seq2")?.as_bytes();
            Ok(RunOutput::Integer(
                bio::bioinformatics::alignment::edit_distance(seq1, seq2) as i64,
            ))
        }
        "bioinf_hamming_distance" => {
            let seq1 = get_str(p, "seq1")?.as_bytes();
            let seq2 = get_str(p, "seq2")?.as_bytes();
            Ok(RunOutput::Integer(
                bio::bioinformatics::alignment::hamming_distance(seq1, seq2) as i64,
            ))
        }
        "alignment_gc_content" => {
            let seq = get_str(p, "seq")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::bioinformatics::alignment::alignment_gc_content(seq),
            ))
        }
        "sequence_identity" => {
            let seq1 = get_str(p, "seq1")?.as_bytes();
            let seq2 = get_str(p, "seq2")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::bioinformatics::alignment::sequence_identity(seq1, seq2),
            ))
        }
        "codon_frequency" => {
            let seq = get_str(p, "seq")?.as_bytes();
            Ok(RunOutput::PairVec(
                bio::bioinformatics::alignment::codon_frequency(seq)
                    .into_iter()
                    .map(|(a, b)| (a as f64, b as f64))
                    .collect(),
            ))
        }
        "bioinf_reverse_complement" => {
            let seq = get_str(p, "seq")?.as_bytes();
            Ok(RunOutput::Text(
                String::from_utf8_lossy(&bio::bioinformatics::alignment::reverse_complement(seq))
                    .into_owned(),
            ))
        }
        "bioinf_melting_temperature_basic" => Ok(RunOutput::Scalar(
            bio::bioinformatics::alignment::melting_temperature_basic(
                get_u(p, "a_count")?,
                get_u(p, "t_count")?,
                get_u(p, "g_count")?,
                get_u(p, "c_count")?,
            ),
        )),
        "assembly_n50" => {
            let contig_lengths_v = get_v(p, "contig_lengths")?;
            let contig_lengths: Vec<usize> = contig_lengths_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Integer(
                bio::bioinformatics::assembly::assembly_n50(&contig_lengths) as i64,
            ))
        }
        "n_metric" => {
            let contig_lengths_v = get_v(p, "contig_lengths")?;
            let contig_lengths: Vec<usize> = contig_lengths_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Integer(bio::bioinformatics::assembly::n_metric(
                &contig_lengths,
                get_f(p, "fraction")?,
            ) as i64))
        }
        "l50" => {
            let contig_lengths_v = get_v(p, "contig_lengths")?;
            let contig_lengths: Vec<usize> = contig_lengths_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Integer(
                bio::bioinformatics::assembly::l50(&contig_lengths) as i64,
            ))
        }
        "genome_coverage" => Ok(RunOutput::Scalar(
            bio::bioinformatics::assembly::genome_coverage(
                get_u(p, "total_bases_sequenced")?,
                get_u(p, "genome_size")?,
            ),
        )),
        "lander_waterman" => Ok(RunOutput::Scalar(
            bio::bioinformatics::assembly::lander_waterman(get_f(p, "coverage")?),
        )),
        "expected_contigs" => Ok(RunOutput::Scalar(
            bio::bioinformatics::assembly::expected_contigs(
                get_u(p, "n_reads")?,
                get_u(p, "read_length")?,
                get_u(p, "genome_size")?,
                get_u(p, "overlap")?,
            ),
        )),
        "assembly_completeness" => Ok(RunOutput::Scalar(
            bio::bioinformatics::assembly::assembly_completeness(
                get_u(p, "aligned_bases")?,
                get_u(p, "reference_size")?,
            ),
        )),
        "gc_content_reads" => {
            let reads_strs = get_str(p, "reads")?;
            let reads_parts: Vec<&[u8]> = reads_strs.split(',').map(|s| s.as_bytes()).collect();
            Ok(RunOutput::Scalar(
                bio::bioinformatics::assembly::gc_content_reads(&reads_parts),
            ))
        }
        "ng50" => {
            let contig_lengths_v = get_v(p, "contig_lengths")?;
            let contig_lengths: Vec<usize> = contig_lengths_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Integer(bio::bioinformatics::assembly::ng50(
                &contig_lengths,
                get_u(p, "genome_size")?,
            ) as i64))
        }
        "aunga" => {
            let contig_lengths_v = get_v(p, "contig_lengths")?;
            let contig_lengths: Vec<usize> = contig_lengths_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(bio::bioinformatics::assembly::aunga(
                &contig_lengths,
                get_u(p, "genome_size")?,
            )))
        }
        "misassembly_rate" => Ok(RunOutput::Scalar(
            bio::bioinformatics::assembly::misassembly_rate(
                get_u(p, "misassemblies")?,
                get_u(p, "total_contigs")?,
            ),
        )),
        "chimeric_contig_fraction" => Ok(RunOutput::Scalar(
            bio::bioinformatics::assembly::chimeric_contig_fraction(
                get_u(p, "chimeric")?,
                get_u(p, "total")?,
            ),
        )),
        "contig_size_distribution" => {
            let contig_lengths_v = get_v(p, "contig_lengths")?;
            let contig_lengths: Vec<usize> = contig_lengths_v.iter().map(|&x| x as usize).collect();
            {
                let r = bio::bioinformatics::assembly::contig_size_distribution(&contig_lengths);
                Ok(RunOutput::Vector(vec![r.0, r.1, r.2 as f64, r.3 as f64]))
            }
        }
        "expected_gap_count" => Ok(RunOutput::Scalar(
            bio::bioinformatics::assembly::expected_gap_count(
                get_f(p, "coverage")?,
                get_u(p, "genome_size")?,
                get_u(p, "read_length")?,
            ),
        )),
        "principal_component_variance" => Ok(RunOutput::Scalar(
            bio::bioinformatics::expression::principal_component_variance(
                get_v(p, "eigenvalues")?,
                get_u(p, "component")?,
            ),
        )),
        "manhattan_distance_features" => Ok(RunOutput::Scalar(
            bio::bioinformatics::expression::manhattan_distance_features(
                get_v(p, "a")?,
                get_v(p, "b")?,
            ),
        )),
        "euclidean_distance_features" => Ok(RunOutput::Scalar(
            bio::bioinformatics::expression::euclidean_distance_features(
                get_v(p, "a")?,
                get_v(p, "b")?,
            ),
        )),
        "pearson_correlation" => Ok(RunOutput::Scalar(
            bio::bioinformatics::expression::pearson_correlation(get_v(p, "x")?, get_v(p, "y")?),
        )),
        "fold_change" => Ok(RunOutput::Scalar(
            bio::bioinformatics::expression::fold_change(
                get_f(p, "treatment")?,
                get_f(p, "control")?,
            ),
        )),
        "log2_fold_change" => Ok(RunOutput::Scalar(
            bio::bioinformatics::expression::log2_fold_change(
                get_f(p, "treatment")?,
                get_f(p, "control")?,
            ),
        )),
        "rpkm" => Ok(RunOutput::Scalar(bio::bioinformatics::expression::rpkm(
            get_f(p, "read_count")?,
            get_f(p, "gene_length_kb")?,
            get_f(p, "total_reads_millions")?,
        ))),
        "tpm" => Ok(RunOutput::Scalar(bio::bioinformatics::expression::tpm(
            get_f(p, "read_count")?,
            get_f(p, "gene_length_kb")?,
            get_f(p, "sum_rpk")?,
        ))),
        "fpkm" => Ok(RunOutput::Scalar(bio::bioinformatics::expression::fpkm(
            get_f(p, "fragment_count")?,
            get_f(p, "gene_length_kb")?,
            get_f(p, "total_fragments_millions")?,
        ))),
        "deseq2_size_factor" => Ok(RunOutput::Scalar(
            bio::bioinformatics::expression::deseq2_size_factor(
                get_v(p, "counts")?,
                get_v(p, "geometric_means")?,
            ),
        )),
        "volcano_significant" => Ok(RunOutput::Boolean(
            bio::bioinformatics::expression::volcano_significant(
                get_f(p, "log2fc")?,
                get_f(p, "p_value")?,
                get_f(p, "fc_threshold")?,
                get_f(p, "p_threshold")?,
            ),
        )),
        "phred_to_probability" => Ok(RunOutput::Scalar(
            bio::bioinformatics::quality::phred_to_probability(get_f(p, "phred")?),
        )),
        "probability_to_phred" => Ok(RunOutput::Scalar(
            bio::bioinformatics::quality::probability_to_phred(get_f(p, "p")?),
        )),
        "average_quality" => {
            let qualities = get_str(p, "qualities")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::bioinformatics::quality::average_quality(qualities),
            ))
        }
        "quality_filter" => {
            let qualities = get_str(p, "qualities")?.as_bytes();
            let min_quality = get_i(p, "min_quality")? as u8;
            Ok(RunOutput::Boolean(
                bio::bioinformatics::quality::quality_filter(
                    qualities,
                    min_quality,
                    get_u(p, "window")?,
                    get_f(p, "min_fraction")?,
                ),
            ))
        }
        "expected_errors" => {
            let qualities = get_str(p, "qualities")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::bioinformatics::quality::expected_errors(qualities),
            ))
        }
        "trim_quality" => {
            let qualities = get_str(p, "qualities")?.as_bytes();
            let min_quality = get_i(p, "min_quality")? as u8;
            Ok(RunOutput::Integer(
                bio::bioinformatics::quality::trim_quality(qualities, min_quality) as i64,
            ))
        }
        "bioinf_n50" => {
            let lengths_v = get_v(p, "lengths")?;
            let lengths: Vec<usize> = lengths_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Integer(
                bio::bioinformatics::quality::n50(&lengths) as i64,
            ))
        }
        "bioinf_gc_content" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Scalar(bio::bioinformatics::quality::gc_content(
                sequence,
            )))
        }
        "adapter_match_score" => {
            let read = get_str(p, "read")?.as_bytes();
            let adapter = get_str(p, "adapter")?.as_bytes();
            Ok(RunOutput::Integer(
                bio::bioinformatics::quality::adapter_match_score(read, adapter) as i64,
            ))
        }
        "complexity_dust" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::bioinformatics::quality::complexity_dust(sequence, get_u(p, "window")?),
            ))
        }
        "kmer_frequency" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Text(
                bio::bioinformatics::quality::kmer_frequency(sequence, get_u(p, "k")?)
                    .into_iter()
                    .map(|(v, n)| format!("{}:{n}", String::from_utf8_lossy(&v)))
                    .collect::<Vec<_>>()
                    .join(","),
            ))
        }
        "shannon_entropy_sequence" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::bioinformatics::quality::shannon_entropy_sequence(sequence),
            ))
        }
        "sliding_window_quality" => {
            let qualities = get_str(p, "qualities")?.as_bytes();
            Ok(RunOutput::Vector(
                bio::bioinformatics::quality::sliding_window_quality(
                    qualities,
                    get_u(p, "window")?,
                ),
            ))
        }
        "per_base_quality_distribution" => {
            let quality_matrix_strs = get_str(p, "quality_matrix")?;
            let quality_matrix_vecs: Vec<Vec<u8>> = quality_matrix_strs
                .split(',')
                .map(|s| s.as_bytes().to_vec())
                .collect();
            Ok(RunOutput::PairVec(
                bio::bioinformatics::quality::per_base_quality_distribution(&quality_matrix_vecs),
            ))
        }
        "duplication_rate" => Ok(RunOutput::Scalar(
            bio::bioinformatics::quality::duplication_rate(
                get_u(p, "total_reads")?,
                get_u(p, "unique_reads")?,
            ),
        )),
        "chimera_breakpoint_score" => Ok(RunOutput::Scalar(
            bio::bioinformatics::quality::chimera_breakpoint_score(
                get_u(p, "alignment_a")?,
                get_u(p, "alignment_b")?,
                get_u(p, "read_length")?,
            ),
        )),
        "melting_temperature_basic" => Ok(RunOutput::Scalar(
            bio::bioinformatics::alignment::melting_temperature_basic(
                get_u(p, "a_count")?,
                get_u(p, "t_count")?,
                get_u(p, "g_count")?,
                get_u(p, "c_count")?,
            ),
        )),
        "gc_content" => {
            let s = get_str(p, "sequence")?;
            Ok(RunOutput::Scalar(bio::bioinformatics::quality::gc_content(
                s.as_bytes(),
            )))
        }
        "hamming_distance" => {
            let s1 = get_str(p, "seq1")?;
            let s2 = get_str(p, "seq2")?;
            Ok(RunOutput::Integer(
                bio::bioinformatics::alignment::hamming_distance(s1.as_bytes(), s2.as_bytes())
                    as i64,
            ))
        }
        "reverse_complement" => {
            let s = get_str(p, "sequence")?;
            let rc = bio::bioinformatics::alignment::reverse_complement(s.as_bytes());
            Ok(RunOutput::Text(String::from_utf8_lossy(&rc).into_owned()))
        }
        "n50" => {
            let v = get_v(p, "lengths")?;
            let lengths: Vec<usize> = v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Integer(
                bio::bioinformatics::quality::n50(&lengths) as i64,
            ))
        }
        "benjamini_hochberg" => {
            let v = get_v(p, "p_values")?;
            let mut pv: Vec<(usize, f64)> = v.iter().enumerate().map(|(i, &x)| (i, x)).collect();
            let result = bio::bioinformatics::expression::benjamini_hochberg(&mut pv);
            let flat: Vec<f64> = result
                .iter()
                .flat_map(|(i, q)| vec![*i as f64, *q])
                .collect();
            Ok(RunOutput::Vector(flat))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
