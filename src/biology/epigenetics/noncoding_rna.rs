pub fn mirna_target_repression(
    mirna_conc: f64,
    target_mrna: f64,
    kd: f64,
    max_repression: f64,
) -> f64 {
    let bound_fraction = mirna_conc / (kd + mirna_conc);
    target_mrna * (1.0 - max_repression * bound_fraction)
}

pub fn mirna_seed_match_score(
    seed_matches: usize,
    three_prime_pairing: f64,
    site_accessibility: f64,
) -> f64 {
    seed_matches as f64 * three_prime_pairing * site_accessibility
}

pub fn lncrna_scaffold_activity(
    lncrna_conc: f64,
    protein_a: f64,
    protein_b: f64,
    kd_a: f64,
    kd_b: f64,
) -> f64 {
    let bound_a = lncrna_conc * protein_a / (kd_a + protein_a);
    let bound_b = lncrna_conc * protein_b / (kd_b + protein_b);
    bound_a * bound_b / lncrna_conc.max(1e-30)
}

pub fn xist_silencing_spread(
    xist_expression: f64,
    distance_from_xic: f64,
    spread_rate: f64,
) -> f64 {
    xist_expression * (-distance_from_xic / spread_rate.max(1e-30)).exp()
}

pub fn pirna_transposon_silencing(
    pirna_conc: f64,
    transposon_copies: f64,
    silencing_efficiency: f64,
) -> f64 {
    let silenced = transposon_copies * (1.0 - (-silencing_efficiency * pirna_conc).exp());
    transposon_copies - silenced
}

pub fn circular_rna_mirna_sponge(
    circrna: f64,
    binding_sites: f64,
    mirna_total: f64,
    kd: f64,
) -> f64 {
    let sponged = circrna * binding_sites * mirna_total / (kd + mirna_total);
    (mirna_total - sponged).max(0.0)
}

pub fn rnai_knockdown_efficiency(
    sirna_conc: f64,
    target_mrna: f64,
    risc_activity: f64,
    kd: f64,
) -> f64 {
    let silencing = risc_activity * sirna_conc / (kd + sirna_conc);
    target_mrna * (1.0 - silencing)
}

pub fn enhancer_rna_activity(
    erna_level: f64,
    enhancer_activity_base: f64,
    amplification: f64,
) -> f64 {
    enhancer_activity_base * (1.0 + amplification * erna_level)
}

pub fn antisense_rna_regulation(
    sense_mrna: f64,
    antisense_rna: f64,
    duplex_rate: f64,
    degradation_rate: f64,
) -> f64 {
    let duplex_formation = duplex_rate * sense_mrna * antisense_rna;
    sense_mrna - duplex_formation / degradation_rate.max(1e-30)
}

pub fn ncrna_mediated_methylation(
    ncrna_guide: f64,
    target_region_accessibility: f64,
    dnmt_activity: f64,
) -> f64 {
    ncrna_guide * target_region_accessibility * dnmt_activity / (1.0 + ncrna_guide)
}
