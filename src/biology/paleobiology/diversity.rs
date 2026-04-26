pub fn faith_phylogenetic_diversity(branch_lengths: &[f64]) -> f64 {
    branch_lengths.iter().sum()
}

pub fn mean_pairwise_distance(distances: &[f64], n_taxa: usize) -> f64 {
    let pairs = n_taxa * (n_taxa - 1) / 2;
    if pairs == 0 {
        return 0.0;
    }
    distances.iter().sum::<f64>() / pairs as f64
}

pub fn net_relatedness_index(mpd_observed: f64, mpd_null_mean: f64, mpd_null_sd: f64) -> f64 {
    -(mpd_observed - mpd_null_mean) / mpd_null_sd.max(1e-30)
}

pub fn nearest_taxon_index(mntd_observed: f64, mntd_null_mean: f64, mntd_null_sd: f64) -> f64 {
    -(mntd_observed - mntd_null_mean) / mntd_null_sd.max(1e-30)
}

pub fn evolutionary_distinctiveness(terminal_branch_length: f64, clade_size: usize) -> f64 {
    terminal_branch_length / clade_size.max(1) as f64
}

pub fn phylogenetic_species_variability(species_correlation_sum: f64, n_species: usize) -> f64 {
    let n = n_species as f64;
    if n <= 0.0 {
        return 0.0;
    }
    1.0 - species_correlation_sum / (n * n)
}

pub fn diversification_rate(n_extant: f64, stem_age: f64) -> f64 {
    if stem_age <= 0.0 {
        return 0.0;
    }
    n_extant.max(1.0).ln() / stem_age
}

pub fn lineage_through_time_expected(n0: f64, birth_rate: f64, death_rate: f64, t: f64) -> f64 {
    n0 * ((birth_rate - death_rate) * t).exp()
}

pub fn gamma_statistic(branching_times: &[f64]) -> f64 {
    let n = branching_times.len();
    if n < 3 {
        return 0.0;
    }
    let mut sorted = branching_times.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let total_time = sorted[n - 1];
    let mut t_sum = 0.0;
    for (i, &val) in sorted.iter().enumerate() {
        t_sum += val * (i + 2) as f64;
    }
    let expected = total_time * (n as f64 + 1.0) / 2.0;
    (t_sum / n as f64 - expected) / (total_time / (12.0 * n as f64).sqrt()).max(1e-30)
}
