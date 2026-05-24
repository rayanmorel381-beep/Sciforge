pub fn principal_component_variance(eigenvalues: &[f64], component: usize) -> f64 {
    let total: f64 = eigenvalues.iter().sum();
    if total < 1e-30 || component >= eigenvalues.len() {
        return 0.0;
    }
    eigenvalues[component] / total
}

pub fn manhattan_distance_features(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| (x - y).abs()).sum()
}

pub fn euclidean_distance_features(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len()) as f64;
    if n < 2.0 {
        return 0.0;
    }
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    for i in 0..n as usize {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    sxy / (sxx * syy).sqrt().max(1e-30)
}

pub fn fold_change(treatment: f64, control: f64) -> f64 {
    if control.abs() < 1e-30 {
        return 0.0;
    }
    treatment / control
}

pub fn log2_fold_change(treatment: f64, control: f64) -> f64 {
    if treatment < 1e-30 || control < 1e-30 {
        return 0.0;
    }
    (treatment / control).log2()
}

pub fn rpkm(read_count: f64, gene_length_kb: f64, total_reads_millions: f64) -> f64 {
    read_count / (gene_length_kb * total_reads_millions).max(1e-30)
}

pub fn tpm(read_count: f64, gene_length_kb: f64, sum_rpk: f64) -> f64 {
    let rpk = read_count / gene_length_kb.max(1e-30);
    rpk / sum_rpk.max(1e-30) * 1e6
}

pub fn fpkm(fragment_count: f64, gene_length_kb: f64, total_fragments_millions: f64) -> f64 {
    fragment_count / (gene_length_kb * total_fragments_millions).max(1e-30)
}

pub fn deseq2_size_factor(counts: &[f64], geometric_means: &[f64]) -> f64 {
    let n = counts.len().min(geometric_means.len());
    let mut ratios: Vec<f64> = (0..n)
        .filter(|&i| geometric_means[i] > 1e-30)
        .map(|i| counts[i] / geometric_means[i])
        .collect();
    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if ratios.is_empty() {
        return 1.0;
    }
    ratios[ratios.len() / 2]
}

pub fn benjamini_hochberg(p_values: &mut [(usize, f64)]) -> Vec<(usize, f64)> {
    let m = p_values.len() as f64;
    p_values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut adjusted = Vec::with_capacity(p_values.len());
    let mut prev_adj = 1.0;
    for i in (0..p_values.len()).rev() {
        let rank = (i + 1) as f64;
        let adj = (p_values[i].1 * m / rank).min(prev_adj).min(1.0);
        adjusted.push((p_values[i].0, adj));
        prev_adj = adj;
    }
    adjusted.reverse();
    adjusted
}

pub fn volcano_significant(log2fc: f64, p_value: f64, fc_threshold: f64, p_threshold: f64) -> bool {
    log2fc.abs() >= fc_threshold && p_value <= p_threshold
}
