pub fn allele_frequency_selection(p: f64, w_aa: f64, w_ab: f64, w_bb: f64) -> f64 {
    let q = 1.0 - p;
    let w_bar = p * p * w_aa + 2.0 * p * q * w_ab + q * q * w_bb;
    if w_bar < 1e-30 {
        return p;
    }
    p * (p * w_aa + q * w_ab) / w_bar
}

pub fn selection_iterate(p0: f64, w_aa: f64, w_ab: f64, w_bb: f64, generations: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(generations + 1);
    let mut p = p0;
    result.push(p);
    for _ in 0..generations {
        p = allele_frequency_selection(p, w_aa, w_ab, w_bb);
        result.push(p);
    }
    result
}

pub fn selection_coefficient(w_mutant: f64, w_wildtype: f64) -> f64 {
    1.0 - w_mutant / w_wildtype
}

pub fn mutation_selection_balance(mu: f64, s: f64) -> f64 {
    mu / s
}

pub fn mutation_selection_balance_recessive(mu: f64, s: f64) -> f64 {
    (mu / s).sqrt()
}

pub fn frequency_dependent_fitness(p: f64, a: f64, b: f64) -> f64 {
    a + b * (1.0 - 2.0 * p)
}

pub fn heterozygote_advantage_equilibrium(s: f64, t: f64) -> f64 {
    t / (s + t)
}

pub fn disruptive_selection(
    p: f64,
    w_aa: f64,
    w_ab: f64,
    w_bb: f64,
    generations: usize,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(generations + 1);
    let mut freq = p;
    result.push(freq);
    for _ in 0..generations {
        freq = allele_frequency_selection(freq, w_aa, w_ab, w_bb);
        result.push(freq);
    }
    result
}

pub fn truncation_selection(mean: f64, variance: f64, threshold: f64) -> f64 {
    let z = (threshold - mean) / variance.sqrt().max(1e-30);
    let phi = (-0.5 * z * z).exp() / (2.0 * std::f64::consts::PI).sqrt();
    let big_phi = 0.5 * (1.0 + erf_approx(z / std::f64::consts::SQRT_2));
    let selection_intensity = phi / (1.0 - big_phi).max(1e-30);
    mean + selection_intensity * variance.sqrt()
}

fn erf_approx(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.3275911 * x.abs());
    let poly = t
        * (0.254829592
            + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let result = 1.0 - poly * (-x * x).exp();
    if x >= 0.0 { result } else { -result }
}

pub fn response_to_selection(heritability: f64, selection_differential: f64) -> f64 {
    heritability * selection_differential
}
