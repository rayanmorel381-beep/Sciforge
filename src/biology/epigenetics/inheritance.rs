pub fn epigenetic_inheritance_probability(maintenance: f64, generations: usize) -> f64 {
    maintenance.powi(generations as i32)
}

pub fn transgenerational_decay(mark: f64, reset_rate: f64, generations: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(generations + 1);
    let mut m = mark;
    result.push(m);
    for _ in 0..generations {
        m *= 1.0 - reset_rate;
        result.push(m);
    }
    result
}

pub fn epimutation_rate(changes: usize, sites: usize, generations: usize) -> f64 {
    if sites == 0 || generations == 0 {
        return 0.0;
    }
    changes as f64 / (sites as f64 * generations as f64)
}

pub fn epiallele_frequency(
    fitness_epi: f64,
    fitness_normal: f64,
    freq: f64,
    generations: usize,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(generations + 1);
    let mut p = freq;
    result.push(p);
    for _ in 0..generations {
        let w_bar = p * fitness_epi + (1.0 - p) * fitness_normal;
        p = p * fitness_epi / w_bar.max(1e-30);
        p = p.clamp(0.0, 1.0);
        result.push(p);
    }
    result
}

pub fn chromatin_state_transition(state: &[f64], transition_matrix: &[Vec<f64>]) -> Vec<f64> {
    let n = state.len();
    let mut new_state = vec![0.0; n];
    for (i, ns) in new_state.iter_mut().enumerate() {
        if i < transition_matrix.len() {
            for (&sj, &tmij) in state.iter().zip(transition_matrix[i].iter()) {
                *ns += tmij * sj;
            }
        }
    }
    new_state
}

pub fn imprinting_expression(maternal: f64, paternal: f64, imprint_maternal: bool) -> f64 {
    if imprint_maternal { paternal } else { maternal }
}

pub fn paramutation(
    strong_allele: f64,
    weak_allele: f64,
    conversion_rate: f64,
    generations: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(generations + 1);
    let mut s = strong_allele;
    let mut w = weak_allele;
    result.push((s, w));
    for _ in 0..generations {
        let conversion = conversion_rate * s * w;
        s -= conversion;
        w += conversion;
        s = s.clamp(0.0, 1.0);
        w = w.clamp(0.0, 1.0);
        result.push((s, w));
    }
    result
}

pub fn metastable_epiallele(
    base_expression: f64,
    methylation: f64,
    stochastic_variance: f64,
) -> f64 {
    base_expression * (1.0 - methylation) + stochastic_variance * methylation * (1.0 - methylation)
}

pub fn epigenetic_clock(methylation_sites: &[f64], coefficients: &[f64], intercept: f64) -> f64 {
    let n = methylation_sites.len().min(coefficients.len());
    let mut age = intercept;
    for i in 0..n {
        age += methylation_sites[i] * coefficients[i];
    }
    age.max(0.0)
}

pub fn environmental_epigenetic_response(
    stress: f64,
    sensitivity: f64,
    methylation_change_rate: f64,
    baseline_methylation: f64,
) -> f64 {
    let response =
        methylation_change_rate * (1.0 / (1.0 + (-sensitivity * (stress - 0.5)).exp()) - 0.5);
    (baseline_methylation + response).clamp(0.0, 1.0)
}
