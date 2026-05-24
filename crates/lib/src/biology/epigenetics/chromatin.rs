pub fn chromatin_accessibility(
    open_fraction: f64,
    remodeler_activity: f64,
    histone_density: f64,
) -> f64 {
    open_fraction + remodeler_activity * (1.0 - open_fraction) / (1.0 + histone_density)
}

pub fn nucleosome_occupancy(dna_affinity: f64, histone_conc: f64, competitor_conc: f64) -> f64 {
    dna_affinity * histone_conc / (1.0 + dna_affinity * histone_conc + competitor_conc)
}

pub fn histone_mark_propagation(
    mark_fraction: f64,
    write_rate: f64,
    erase_rate: f64,
    dt: f64,
) -> f64 {
    let d_mark = write_rate * (1.0 - mark_fraction) - erase_rate * mark_fraction;
    (mark_fraction + d_mark * dt).clamp(0.0, 1.0)
}

pub fn polycomb_silencing(pc_binding: f64, h3k27me3: f64, cooperative_factor: f64) -> f64 {
    let binding = pc_binding * h3k27me3.powf(cooperative_factor);
    binding / (1.0 + binding)
}

pub fn trithorax_activation(trx_binding: f64, h3k4me3: f64, cooperative_factor: f64) -> f64 {
    let binding = trx_binding * h3k4me3.powf(cooperative_factor);
    binding / (1.0 + binding)
}

pub fn chromatin_loop_probability(distance_bp: f64, persistence_length_bp: f64) -> f64 {
    let n = distance_bp / persistence_length_bp;
    if n <= 0.0 {
        return 0.0;
    }
    (3.0 / (2.0 * std::f64::consts::PI * n)).powf(1.5)
}

pub fn tad_insulation_score(contacts_within: f64, contacts_across: f64) -> f64 {
    contacts_within / (contacts_within + contacts_across).max(1e-30)
}

pub fn enhancer_promoter_contact(
    distance: f64,
    cohesin_loading: f64,
    ctcf_binding: f64,
    decay_length: f64,
) -> f64 {
    let base_contact = (-distance / decay_length).exp();
    base_contact * (1.0 + cohesin_loading * ctcf_binding)
}

pub fn bivalent_resolution(h3k4me3: f64, h3k27me3: f64, differentiation_signal: f64) -> (f64, f64) {
    let active = h3k4me3 * differentiation_signal / (1.0 + h3k27me3);
    let silent = h3k27me3 * (1.0 - differentiation_signal) / (1.0 + h3k4me3);
    (active, silent)
}

pub fn heterochromatin_spread(
    hp1_conc: f64,
    h3k9me3: f64,
    barrier_strength: f64,
    distance: f64,
) -> f64 {
    let spread = hp1_conc * h3k9me3 * (-distance / 10.0).exp();
    spread / (1.0 + barrier_strength)
}

pub fn atac_seq_signal(fragment_count: f64, total_reads: f64, region_length_bp: f64) -> f64 {
    fragment_count * 1e9 / (total_reads * region_length_bp).max(1e-30)
}
