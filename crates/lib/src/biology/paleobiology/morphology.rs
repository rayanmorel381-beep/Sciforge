pub fn morphological_disparity(trait_values: &[Vec<f64>]) -> f64 {
    if trait_values.is_empty() {
        return 0.0;
    }
    let n = trait_values.len() as f64;
    let dims = trait_values[0].len();
    let centroid: Vec<f64> = (0..dims)
        .map(|d| trait_values.iter().map(|t| t[d]).sum::<f64>() / n)
        .collect();
    let sum_sq: f64 = trait_values
        .iter()
        .map(|t| {
            t.iter()
                .zip(centroid.iter())
                .map(|(&a, &b)| (a - b) * (a - b))
                .sum::<f64>()
        })
        .sum();
    sum_sq / (n - 1.0)
}

pub fn rarefied_diversity(abundances: &[usize], sample_size: usize) -> f64 {
    let n: usize = abundances.iter().sum();
    if sample_size >= n {
        return abundances.iter().filter(|&&a| a > 0).count() as f64;
    }
    abundances
        .iter()
        .filter(|&&ni| ni > 0)
        .map(|&ni| 1.0 - hypergeometric_complement(n, ni, sample_size))
        .sum()
}

fn hypergeometric_complement(n: usize, ni: usize, m: usize) -> f64 {
    let mut log_result = 0.0;
    for k in 0..m {
        log_result += ((n - ni - k) as f64).ln() - ((n - k) as f64).ln();
    }
    log_result.exp()
}

pub fn foote_boundary_crosser_rate(n_bt: f64, n_fl: f64) -> f64 {
    -(n_fl / n_bt).ln()
}

pub fn completeness_index(known_intervals: f64, total_range: f64) -> f64 {
    known_intervals / total_range
}

pub fn lazarus_ratio(lazarus_taxa: f64, total_taxa: f64) -> f64 {
    lazarus_taxa / total_taxa
}

pub fn body_size_cope_trend(sizes: &[f64]) -> f64 {
    if sizes.len() < 2 {
        return 0.0;
    }
    let n = sizes.len() as f64;
    let x_mean = (n - 1.0) / 2.0;
    let y_mean: f64 = sizes.iter().sum::<f64>() / n;
    let num: f64 = sizes
        .iter()
        .enumerate()
        .map(|(i, &y)| (i as f64 - x_mean) * (y - y_mean))
        .sum();
    let den: f64 = (0..sizes.len()).map(|i| (i as f64 - x_mean).powi(2)).sum();
    if den == 0.0 { 0.0 } else { num / den }
}

pub fn morphospace_range(trait_values: &[f64]) -> f64 {
    let max = trait_values
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let min = trait_values.iter().cloned().fold(f64::INFINITY, f64::min);
    max - min
}

pub fn morphospace_volume(ranges: &[f64]) -> f64 {
    ranges.iter().product()
}

pub fn pairwise_morphological_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y) * (x - y))
        .sum::<f64>()
        .sqrt()
}

pub fn evolutionary_rate_darwin(size_initial: f64, size_final: f64, time_myr: f64) -> f64 {
    (size_final.ln() - size_initial.ln()) / time_myr
}

pub fn evolutionary_rate_haldane(
    size_initial: f64,
    size_final: f64,
    time_generations: f64,
    pooled_sd: f64,
) -> f64 {
    ((size_final.ln() - size_initial.ln()) / pooled_sd) / time_generations
}

pub fn taphonomic_bias(original_richness: f64, preservation_prob: f64) -> f64 {
    original_richness * (1.0 - (1.0 - preservation_prob).powf(original_richness))
}

pub fn ghost_lineage_duration(first_appearance: f64, inferred_origin: f64) -> f64 {
    (first_appearance - inferred_origin).abs()
}

pub fn disparity_centroid_distance(taxon: &[f64], centroid: &[f64]) -> f64 {
    taxon
        .iter()
        .zip(centroid.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}
