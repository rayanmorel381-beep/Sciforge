pub fn broad_sense_heritability(genetic_variance: f64, phenotypic_variance: f64) -> f64 {
    genetic_variance / phenotypic_variance.max(1e-30)
}

pub fn narrow_sense_heritability(additive_variance: f64, phenotypic_variance: f64) -> f64 {
    additive_variance / phenotypic_variance.max(1e-30)
}

pub fn breeder_equation(heritability: f64, selection_differential: f64) -> f64 {
    heritability * selection_differential
}

pub fn selection_differential(mean_selected: f64, mean_population: f64) -> f64 {
    mean_selected - mean_population
}

pub fn additive_genetic_variance(allele_freq: f64, allele_effect: f64) -> f64 {
    2.0 * allele_freq * (1.0 - allele_freq) * allele_effect * allele_effect
}

pub fn dominance_variance(allele_freq: f64, dominance_deviation: f64) -> f64 {
    (2.0 * allele_freq * (1.0 - allele_freq) * dominance_deviation).powi(2)
}

pub fn qtl_effect_from_means(mean_aa: f64, mean_ab: f64, mean_bb: f64) -> (f64, f64) {
    let a = (mean_bb - mean_aa) / 2.0;
    let d = mean_ab - (mean_aa + mean_bb) / 2.0;
    (a, d)
}

pub fn phenotypic_variance_components(va: f64, vd: f64, ve: f64, vi: f64) -> f64 {
    va + vd + ve + vi
}

pub fn realized_heritability(response: f64, selection_differential: f64) -> f64 {
    response / selection_differential.max(1e-30)
}

pub fn mid_parent_regression(
    parent1: f64,
    parent2: f64,
    heritability: f64,
    population_mean: f64,
) -> f64 {
    let mid_parent = (parent1 + parent2) / 2.0;
    population_mean + heritability * (mid_parent - population_mean)
}

pub fn lander_botstein_lod(n: usize, r_squared: f64) -> f64 {
    let n_f = n as f64;
    n_f / 2.0 * (1.0 - r_squared).max(1e-30).ln().abs() / 10.0_f64.ln()
}

pub fn polygenic_score(effects: &[f64], genotypes: &[f64]) -> f64 {
    let n = effects.len().min(genotypes.len());
    let mut score = 0.0;
    for i in 0..n {
        score += effects[i] * genotypes[i];
    }
    score
}

pub fn falconer_liability_threshold(prevalence: f64) -> f64 {
    fn inv_normal_approx(p: f64) -> f64 {
        let p_c = p.clamp(1e-10, 1.0 - 1e-10);
        let t = (-2.0 * p_c.ln()).sqrt();
        t - (2.515517 + 0.802853 * t + 0.010328 * t * t)
            / (1.0 + 1.432788 * t + 0.189269 * t * t + 0.001308 * t * t * t)
    }
    inv_normal_approx(1.0 - prevalence)
}

pub fn snp_heritability(
    beta_squared_sum: f64,
    variance_explained: f64,
    phenotypic_variance: f64,
) -> f64 {
    (beta_squared_sum * variance_explained) / phenotypic_variance.max(1e-30)
}
