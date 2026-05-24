pub fn cpg_methylation_level(methylated: usize, total_cpg: usize) -> f64 {
    if total_cpg == 0 {
        return 0.0;
    }
    methylated as f64 / total_cpg as f64
}

pub fn methylation_decay(
    level: f64,
    dilution_rate: f64,
    maintenance_efficiency: f64,
    generations: usize,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(generations + 1);
    let mut m = level;
    result.push(m);
    for _ in 0..generations {
        m = m * maintenance_efficiency * (1.0 - dilution_rate);
        m = m.clamp(0.0, 1.0);
        result.push(m);
    }
    result
}

pub fn de_novo_methylation(unmethylated: f64, rate: f64, dt: f64) -> f64 {
    unmethylated * rate * dt
}

pub fn bisulfite_conversion_efficiency(converted_c: usize, total_c: usize) -> f64 {
    if total_c == 0 {
        return 0.0;
    }
    converted_c as f64 / total_c as f64
}

pub fn methylation_entropy(profile: &[f64]) -> f64 {
    let mut entropy = 0.0;
    for &p in profile {
        if p > 0.0 && p < 1.0 {
            entropy -= p * p.log2() + (1.0 - p) * (1.0 - p).log2();
        }
    }
    if profile.is_empty() {
        0.0
    } else {
        entropy / profile.len() as f64
    }
}

pub fn hydroxymethylation_rate(methylation: f64, tet_activity: f64) -> f64 {
    tet_activity * methylation
}

pub fn tet_oxidation_cascade(mc: f64, tet: f64, dt: f64) -> (f64, f64, f64) {
    let hmc = tet * mc * dt;
    let fc = tet * hmc * dt * 0.5;
    let cac = tet * fc * dt * 0.25;
    (hmc, fc, cac)
}

pub fn dnmt_processivity(
    initial_methylation: &[f64],
    processivity: f64,
    direction_forward: bool,
) -> Vec<f64> {
    let mut result = initial_methylation.to_vec();
    let mut prob = 1.0;
    if direction_forward {
        for ri in result.iter_mut() {
            *ri = (*ri + prob * (1.0 - *ri) * 0.5).min(1.0);
            prob *= processivity;
        }
    } else {
        for ri in result.iter_mut().rev() {
            *ri = (*ri + prob * (1.0 - *ri) * 0.5).min(1.0);
            prob *= processivity;
        }
    }
    result
}

pub fn cpg_island_density(sequence_length: usize, cpg_count: usize, expected_cpg: f64) -> f64 {
    if expected_cpg < 1e-30 {
        return 0.0;
    }
    cpg_count as f64 / (expected_cpg * sequence_length as f64 / sequence_length as f64)
}

pub fn age_methylation_predictor(cpg_beta_values: &[f64], weights: &[f64], offset: f64) -> f64 {
    let mut predicted = offset;
    for (&bv, &w) in cpg_beta_values.iter().zip(weights.iter()) {
        predicted += bv * w;
    }
    predicted.max(0.0)
}
