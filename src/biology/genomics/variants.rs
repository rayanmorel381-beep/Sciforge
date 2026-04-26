pub fn snp_allele_frequency(alt_count: usize, total_alleles: usize) -> f64 {
    alt_count as f64 / total_alleles.max(1) as f64
}

pub fn minor_allele_frequency(allele_freq: f64) -> f64 {
    allele_freq.min(1.0 - allele_freq)
}

pub fn hardy_weinberg_expected(p: f64) -> (f64, f64, f64) {
    let q = 1.0 - p;
    (p * p, 2.0 * p * q, q * q)
}

pub fn hardy_weinberg_chi_squared(observed: &[f64; 3], expected: &[f64; 3]) -> f64 {
    let mut chi2 = 0.0;
    for i in 0..3 {
        if expected[i] > 0.0 {
            chi2 += (observed[i] - expected[i]).powi(2) / expected[i];
        }
    }
    chi2
}

pub fn ti_tv_ratio(transitions: usize, transversions: usize) -> f64 {
    transitions as f64 / transversions.max(1) as f64
}

pub fn heterozygosity(allele_freqs: &[f64]) -> f64 {
    let sum_sq: f64 = allele_freqs.iter().map(|f| f * f).sum();
    1.0 - sum_sq
}

pub fn fst_weir_cockerham(het_within: f64, het_total: f64) -> f64 {
    1.0 - het_within / het_total.max(1e-30)
}

pub fn linkage_disequilibrium(freq_ab: f64, freq_a: f64, freq_b: f64) -> f64 {
    freq_ab - freq_a * freq_b
}

pub fn r_squared_ld(d: f64, freq_a: f64, freq_b: f64) -> f64 {
    let denom = freq_a * (1.0 - freq_a) * freq_b * (1.0 - freq_b);
    if denom <= 0.0 {
        return 0.0;
    }
    d * d / denom
}

pub fn d_prime(d: f64, freq_a: f64, freq_b: f64) -> f64 {
    let d_max = if d >= 0.0 {
        (freq_a * (1.0 - freq_b)).min((1.0 - freq_a) * freq_b)
    } else {
        (freq_a * freq_b).min((1.0 - freq_a) * (1.0 - freq_b))
    };
    d / d_max.max(1e-30)
}

pub fn indel_frameshift(indel_length: i64) -> bool {
    indel_length % 3 != 0
}

pub fn copy_number_variant_dosage(reads_sample: f64, reads_reference: f64, ploidy: f64) -> f64 {
    ploidy * reads_sample / reads_reference.max(1e-30)
}
