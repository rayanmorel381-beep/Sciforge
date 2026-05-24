pub fn spectral_count_nsaf(spectral_count: f64, protein_length: f64, total_nsaf: f64) -> f64 {
    let saf = spectral_count / protein_length.max(1e-30);
    saf / total_nsaf.max(1e-30)
}

pub fn ibaq(total_intensity: f64, num_observable_peptides: usize) -> f64 {
    total_intensity / num_observable_peptides.max(1) as f64
}

pub fn lfq_ratio(intensity_sample: f64, intensity_reference: f64) -> f64 {
    intensity_sample / intensity_reference.max(1e-30)
}

pub fn tmt_reporter_ratio(reporter_intensity: f64, reference_channel: f64) -> f64 {
    reporter_intensity / reference_channel.max(1e-30)
}

pub fn silac_ratio(heavy: f64, light: f64) -> f64 {
    heavy / light.max(1e-30)
}

pub fn protein_fdr(decoy_hits: f64, target_hits: f64) -> f64 {
    decoy_hits / target_hits.max(1e-30)
}

pub fn mascot_ion_score(observed: f64, expected: f64) -> f64 {
    -10.0 * expected.max(1e-30).log10() + 10.0 * observed.max(1e-30).log10()
}

pub fn em_pai(observed_peptides: usize, observable_peptides: usize) -> f64 {
    10.0_f64.powf(observed_peptides as f64 / observable_peptides.max(1) as f64) - 1.0
}

pub fn protein_coverage(identified_residues: usize, total_residues: usize) -> f64 {
    identified_residues as f64 / total_residues.max(1) as f64
}

pub fn xcorr_normalized(xcorr: f64, peptide_length: usize) -> f64 {
    xcorr / peptide_length.max(1) as f64
}

pub fn missed_cleavage_rate(missed: usize, total_peptides: usize) -> f64 {
    missed as f64 / total_peptides.max(1) as f64
}
