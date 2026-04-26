pub fn gene_density(genes: usize, region_length_mb: f64) -> f64 {
    genes as f64 / region_length_mb.max(1e-30)
}

pub fn cpg_enrichment(cpg_count: usize, c_count: usize, g_count: usize, length: usize) -> f64 {
    let expected = c_count as f64 * g_count as f64 / length.max(1) as f64;
    cpg_count as f64 / expected.max(1e-30)
}

pub fn codon_adaptation_index(codon_weights: &[f64]) -> f64 {
    if codon_weights.is_empty() {
        return 0.0;
    }
    let sum_ln: f64 = codon_weights.iter().map(|w| w.max(1e-30).ln()).sum();
    (sum_ln / codon_weights.len() as f64).exp()
}

pub fn enc_wright(codon_family_homozygosities: &[f64]) -> f64 {
    if codon_family_homozygosities.is_empty() {
        return 61.0;
    }
    let mean_f: f64 =
        codon_family_homozygosities.iter().sum::<f64>() / codon_family_homozygosities.len() as f64;
    if mean_f <= 0.0 {
        return 61.0;
    }
    2.0 + 9.0 / mean_f + 1.0 / mean_f + 5.0 / mean_f + 3.0 / mean_f
}

pub fn repeat_density(repeat_bases: usize, total_bases: usize) -> f64 {
    repeat_bases as f64 / total_bases.max(1) as f64
}

pub fn synteny_score(conserved_blocks: usize, total_genes: usize) -> f64 {
    conserved_blocks as f64 / total_genes.max(1) as f64
}

pub fn n50(contig_lengths: &mut [f64]) -> f64 {
    contig_lengths.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let total: f64 = contig_lengths.iter().sum();
    let half = total / 2.0;
    let mut cumulative = 0.0;
    for &len in contig_lengths.iter() {
        cumulative += len;
        if cumulative >= half {
            return len;
        }
    }
    0.0
}

pub fn genome_completeness_busco(complete: usize, fragmented: usize, total_buscos: usize) -> f64 {
    (complete as f64 + 0.5 * fragmented as f64) / total_buscos.max(1) as f64
}

pub fn ka_ks_ratio(
    nonsynonymous_subs: f64,
    synonymous_subs: f64,
    nonsynonymous_sites: f64,
    synonymous_sites: f64,
) -> f64 {
    let ka = nonsynonymous_subs / nonsynonymous_sites.max(1e-30);
    let ks = synonymous_subs / synonymous_sites.max(1e-30);
    ka / ks.max(1e-30)
}

pub fn gc_isochore(gc_content: f64) -> &'static str {
    if gc_content < 0.37 {
        "L1"
    } else if gc_content < 0.41 {
        "L2"
    } else if gc_content < 0.46 {
        "H1"
    } else if gc_content < 0.53 {
        "H2"
    } else {
        "H3"
    }
}
