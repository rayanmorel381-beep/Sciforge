pub fn recombination_frequency(recombinants: f64, total_offspring: f64) -> f64 {
    recombinants / total_offspring.max(1e-30)
}

pub fn map_distance_kosambi(recombination_freq: f64) -> f64 {
    0.25 * ((1.0 + 2.0 * recombination_freq) / (1.0 - 2.0 * recombination_freq).max(1e-30)).ln()
}

pub fn map_distance_haldane(recombination_freq: f64) -> f64 {
    -0.5 * (1.0 - 2.0 * recombination_freq).max(1e-30).ln()
}

pub fn haldane_to_recombination(map_distance: f64) -> f64 {
    0.5 * (1.0 - (-2.0 * map_distance).exp())
}

pub fn lod_score(theta: f64, recombinants: usize, non_recombinants: usize) -> f64 {
    let r = recombinants as f64;
    let nr = non_recombinants as f64;
    r * theta.max(1e-30).log10() + nr * (1.0 - theta).max(1e-30).log10()
        - r * 0.5_f64.log10()
        - nr * 0.5_f64.log10()
}

pub fn three_point_cross_distance(class_counts: &[f64; 8]) -> (f64, f64, f64) {
    let total: f64 = class_counts.iter().sum();
    if total <= 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let single_co1 = (class_counts[2] + class_counts[3]) / total;
    let single_co2 = (class_counts[4] + class_counts[5]) / total;
    let double_co = (class_counts[6] + class_counts[7]) / total;
    (
        single_co1 + double_co,
        single_co2 + double_co,
        double_co / (single_co1 + double_co).max(1e-30) / (single_co2 + double_co).max(1e-30),
    )
}

pub fn interference(observed_double_co: f64, expected_double_co: f64) -> f64 {
    1.0 - observed_double_co / expected_double_co.max(1e-30)
}

pub fn chiasma_frequency(recombination_freq: f64) -> f64 {
    2.0 * recombination_freq
}

pub fn synaptonemal_complex_length(chromosome_length_mb: f64, loop_size_kb: f64) -> f64 {
    chromosome_length_mb * 1000.0 / loop_size_kb.max(1e-30)
}
