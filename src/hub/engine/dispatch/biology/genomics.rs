//! Dispatch handler for genomics functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "gene_density" => Ok(RunOutput::Scalar(bio::genomics::annotation::gene_density(
            get_u(p, "genes")?,
            get_f(p, "region_length_mb")?,
        ))),
        "cpg_enrichment" => Ok(RunOutput::Scalar(
            bio::genomics::annotation::cpg_enrichment(
                get_u(p, "cpg_count")?,
                get_u(p, "c_count")?,
                get_u(p, "g_count")?,
                get_u(p, "length")?,
            ),
        )),
        "codon_adaptation_index" => Ok(RunOutput::Scalar(
            bio::genomics::annotation::codon_adaptation_index(get_v(p, "codon_weights")?),
        )),
        "enc_wright" => Ok(RunOutput::Scalar(bio::genomics::annotation::enc_wright(
            get_v(p, "codon_family_homozygosities")?,
        ))),
        "repeat_density" => Ok(RunOutput::Scalar(
            bio::genomics::annotation::repeat_density(
                get_u(p, "repeat_bases")?,
                get_u(p, "total_bases")?,
            ),
        )),
        "synteny_score" => Ok(RunOutput::Scalar(bio::genomics::annotation::synteny_score(
            get_u(p, "conserved_blocks")?,
            get_u(p, "total_genes")?,
        ))),
        "genome_completeness_busco" => Ok(RunOutput::Scalar(
            bio::genomics::annotation::genome_completeness_busco(
                get_u(p, "complete")?,
                get_u(p, "fragmented")?,
                get_u(p, "total_buscos")?,
            ),
        )),
        "ka_ks_ratio" => Ok(RunOutput::Scalar(bio::genomics::annotation::ka_ks_ratio(
            get_f(p, "nonsynonymous_subs")?,
            get_f(p, "synonymous_subs")?,
            get_f(p, "nonsynonymous_sites")?,
            get_f(p, "synonymous_sites")?,
        ))),
        "gc_isochore" => Ok(RunOutput::Text(
            bio::genomics::annotation::gc_isochore(get_f(p, "gc_content")?).to_string(),
        )),
        "pwm_score" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Scalar(bio::genomics::motifs::pwm_score(
                get_m(p, "pwm")?,
                sequence,
            )))
        }
        "pwm_scan" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::PairVec(
                bio::genomics::motifs::pwm_scan(get_m(p, "pwm")?, sequence, get_f(p, "threshold")?)
                    .into_iter()
                    .map(|(a, b)| (a as f64, b))
                    .collect(),
            ))
        }
        "information_content" => Ok(RunOutput::Vector(
            bio::genomics::motifs::information_content(get_m(p, "pwm")?),
        )),
        "total_information" => Ok(RunOutput::Scalar(bio::genomics::motifs::total_information(
            get_m(p, "pwm")?,
        ))),
        "consensus_sequence" => Ok(RunOutput::Text(bio::genomics::motifs::consensus_sequence(
            get_m(p, "pwm")?,
        ))),
        "frequency_matrix" => {
            let sequences_strs = get_str(p, "sequences")?;
            let sequences_parts: Vec<&[u8]> =
                sequences_strs.split(',').map(|s| s.as_bytes()).collect();
            Ok(RunOutput::Matrix(bio::genomics::motifs::frequency_matrix(
                &sequences_parts,
                get_u(p, "length")?,
            )))
        }
        "find_orfs" => Ok(RunOutput::Text(
            bio::genomics::orf::find_orfs(get_str(p, "sequence")?, get_u(p, "min_length")?)
                .into_iter()
                .map(|(a, b, s)| format!("{a}:{b}:{s}"))
                .collect::<Vec<_>>()
                .join(","),
        )),
        "codon_usage" => Ok(RunOutput::Text(
            bio::genomics::orf::codon_usage(get_str(p, "sequence")?)
                .into_iter()
                .map(|(s, n)| format!("{s}:{n}"))
                .collect::<Vec<_>>()
                .join(","),
        )),
        "reading_frame_proteins" => Ok(RunOutput::Text(
            bio::genomics::orf::reading_frame_proteins(get_str(p, "sequence")?, get_u(p, "frame")?)
                .join(","),
        )),
        "genomics_gc_content" => Ok(RunOutput::Scalar(bio::genomics::orf::gc_content(get_str(
            p, "sequence",
        )?))),
        "gc3_content" => Ok(RunOutput::Scalar(bio::genomics::orf::gc3_content(get_str(
            p, "sequence",
        )?))),
        "longest_orf_length" => Ok(RunOutput::Integer(bio::genomics::orf::longest_orf_length(
            get_str(p, "sequence")?,
        ) as i64)),
        "nucleotide_frequency" => {
            let r = bio::genomics::orf::nucleotide_frequency(get_str(p, "sequence")?);
            Ok(RunOutput::Vector(r.to_vec()))
        }
        "genomics_translate" => Ok(RunOutput::Text(bio::genomics::orf::translate(get_str(
            p, "sequence",
        )?))),
        "genomics_reverse_complement" => Ok(RunOutput::Text(
            bio::genomics::orf::reverse_complement(get_str(p, "sequence")?),
        )),
        "kmer_count" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Text(
                bio::genomics::statistics::kmer_count(sequence, get_u(p, "k")?)
                    .into_iter()
                    .map(|(v, n)| format!("{}:{n}", String::from_utf8_lossy(&v)))
                    .collect::<Vec<_>>()
                    .join(","),
            ))
        }
        "gc_skew" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Vector(bio::genomics::statistics::gc_skew(
                sequence,
                get_u(p, "window")?,
            )))
        }
        "cpg_observed_expected" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::genomics::statistics::cpg_observed_expected(sequence),
            ))
        }
        "linguistic_complexity" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::genomics::statistics::linguistic_complexity(sequence),
            ))
        }
        "at_content" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Scalar(bio::genomics::statistics::at_content(
                sequence,
            )))
        }
        "dinucleotide_frequency" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Matrix(
                bio::genomics::statistics::dinucleotide_frequency(sequence)
                    .into_iter()
                    .map(|(a, b, c)| vec![a as f64, b as f64, c])
                    .collect(),
            ))
        }
        "sequence_entropy" => {
            let sequence = get_str(p, "sequence")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::genomics::statistics::sequence_entropy(sequence),
            ))
        }
        "transition_transversion" => {
            let sequence_a = get_str(p, "sequence_a")?.as_bytes();
            let sequence_b = get_str(p, "sequence_b")?.as_bytes();
            Ok(RunOutput::Scalar(
                bio::genomics::statistics::transition_transversion(sequence_a, sequence_b),
            ))
        }
        "snp_allele_frequency" => Ok(RunOutput::Scalar(
            bio::genomics::variants::snp_allele_frequency(
                get_u(p, "alt_count")?,
                get_u(p, "total_alleles")?,
            ),
        )),
        "minor_allele_frequency" => Ok(RunOutput::Scalar(
            bio::genomics::variants::minor_allele_frequency(get_f(p, "allele_freq")?),
        )),
        "hardy_weinberg_expected" => {
            let (a, b, c) = bio::genomics::variants::hardy_weinberg_expected(get_f(p, "p")?);
            Ok(RunOutput::Triple(a, b, c))
        }
        "hardy_weinberg_chi_squared" => {
            let observed_v = get_v(p, "observed")?;
            let observed = [observed_v[0], observed_v[1], observed_v[2]];
            let expected_v = get_v(p, "expected")?;
            let expected = [expected_v[0], expected_v[1], expected_v[2]];
            Ok(RunOutput::Scalar(
                bio::genomics::variants::hardy_weinberg_chi_squared(&observed, &expected),
            ))
        }
        "ti_tv_ratio" => Ok(RunOutput::Scalar(bio::genomics::variants::ti_tv_ratio(
            get_u(p, "transitions")?,
            get_u(p, "transversions")?,
        ))),
        "heterozygosity" => Ok(RunOutput::Scalar(bio::genomics::variants::heterozygosity(
            get_v(p, "allele_freqs")?,
        ))),
        "fst_weir_cockerham" => Ok(RunOutput::Scalar(
            bio::genomics::variants::fst_weir_cockerham(
                get_f(p, "het_within")?,
                get_f(p, "het_total")?,
            ),
        )),
        "linkage_disequilibrium" => Ok(RunOutput::Scalar(
            bio::genomics::variants::linkage_disequilibrium(
                get_f(p, "freq_ab")?,
                get_f(p, "freq_a")?,
                get_f(p, "freq_b")?,
            ),
        )),
        "r_squared_ld" => Ok(RunOutput::Scalar(bio::genomics::variants::r_squared_ld(
            get_f(p, "d")?,
            get_f(p, "freq_a")?,
            get_f(p, "freq_b")?,
        ))),
        "d_prime" => Ok(RunOutput::Scalar(bio::genomics::variants::d_prime(
            get_f(p, "d")?,
            get_f(p, "freq_a")?,
            get_f(p, "freq_b")?,
        ))),
        "indel_frameshift" => Ok(RunOutput::Boolean(
            bio::genomics::variants::indel_frameshift(get_i(p, "indel_length")?),
        )),
        "copy_number_variant_dosage" => Ok(RunOutput::Scalar(
            bio::genomics::variants::copy_number_variant_dosage(
                get_f(p, "reads_sample")?,
                get_f(p, "reads_reference")?,
                get_f(p, "ploidy")?,
            ),
        )),
        "translate" => Ok(RunOutput::Text(bio::genomics::orf::translate(get_str(
            p, "sequence",
        )?))),
        "effective_number_of_codons" => {
            let s = get_str(p, "codons")?;
            let pairs: Vec<(String, usize)> = s
                .split(';')
                .filter_map(|entry| {
                    let mut parts = entry.splitn(2, ':');
                    let codon = parts.next()?.to_string();
                    let count = parts.next()?.parse().ok()?;
                    Some((codon, count))
                })
                .collect();
            Ok(RunOutput::Scalar(
                bio::genomics::orf::effective_number_of_codons(&pairs),
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
