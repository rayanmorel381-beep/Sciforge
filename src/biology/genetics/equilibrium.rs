pub fn hardy_weinberg(p: f64) -> (f64, f64, f64) {
    let q = 1.0 - p;
    (p * p, 2.0 * p * q, q * q)
}

pub fn hardy_weinberg_multi(freqs: &[f64]) -> Vec<Vec<f64>> {
    let n = freqs.len();
    let mut genotypes = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            genotypes[i][j] = if i == j {
                freqs[i] * freqs[j]
            } else {
                2.0 * freqs[i] * freqs[j]
            };
        }
    }
    genotypes
}

pub fn chi_squared_hw(observed: &[f64], p: f64, n_total: f64) -> f64 {
    let (exp_aa, exp_ab, exp_bb) = hardy_weinberg(p);
    let expected = [exp_aa * n_total, exp_ab * n_total, exp_bb * n_total];
    observed
        .iter()
        .zip(expected.iter())
        .map(|(&o, &e)| if e > 0.0 { (o - e) * (o - e) / e } else { 0.0 })
        .sum()
}

pub fn inbreeding_coefficient(h_obs: f64, h_exp: f64) -> f64 {
    if h_exp < 1e-30 {
        return 0.0;
    }
    1.0 - h_obs / h_exp
}

pub fn wahlund_effect(subpop_freqs: &[f64]) -> f64 {
    let n = subpop_freqs.len() as f64;
    let mean_p = subpop_freqs.iter().sum::<f64>() / n;
    subpop_freqs
        .iter()
        .map(|&p| (p - mean_p) * (p - mean_p))
        .sum::<f64>()
        / n
}

pub fn fst(subpop_freqs: &[f64]) -> f64 {
    let n = subpop_freqs.len() as f64;
    let p_bar = subpop_freqs.iter().sum::<f64>() / n;
    let h_s: f64 = subpop_freqs
        .iter()
        .map(|&p| 2.0 * p * (1.0 - p))
        .sum::<f64>()
        / n;
    let h_t = 2.0 * p_bar * (1.0 - p_bar);
    if h_t < 1e-30 {
        return 0.0;
    }
    1.0 - h_s / h_t
}

pub fn nei_genetic_distance(pop1_freqs: &[f64], pop2_freqs: &[f64]) -> f64 {
    let n = pop1_freqs.len().min(pop2_freqs.len());
    let mut jxy = 0.0;
    let mut jx = 0.0;
    let mut jy = 0.0;
    for i in 0..n {
        jxy += pop1_freqs[i] * pop2_freqs[i];
        jx += pop1_freqs[i] * pop1_freqs[i];
        jy += pop2_freqs[i] * pop2_freqs[i];
    }
    let identity = jxy / (jx * jy).sqrt().max(1e-30);
    -(identity.max(1e-30)).ln()
}

pub fn expected_heterozygosity(allele_freqs: &[f64]) -> f64 {
    1.0 - allele_freqs.iter().map(|&p| p * p).sum::<f64>()
}

pub fn allele_frequency_cline(x: f64, center: f64, width: f64) -> f64 {
    1.0 / (1.0 + ((x - center) / width).exp())
}

pub fn effective_population_size(n_males: f64, n_females: f64) -> f64 {
    4.0 * n_males * n_females / (n_males + n_females).max(1.0)
}
