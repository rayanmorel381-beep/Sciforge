pub fn neutral_substitution_rate(mutation_rate: f64) -> f64 {
    mutation_rate
}

pub fn effective_neutral_mutations(total_mutations: usize, fraction_neutral: f64) -> f64 {
    total_mutations as f64 * fraction_neutral
}

pub fn nearly_neutral_boundary(ne: f64) -> f64 {
    1.0 / (2.0 * ne)
}

pub fn tajima_d(pi: f64, theta_w: f64, n: usize) -> f64 {
    let n_f = n as f64;
    let a1: f64 = (1..n).map(|i| 1.0 / i as f64).sum();
    let a2: f64 = (1..n).map(|i| 1.0 / (i as f64).powi(2)).sum();
    let b1 = (n_f + 1.0) / (3.0 * (n_f - 1.0));
    let b2 = 2.0 * (n_f * n_f + n_f + 3.0) / (9.0 * n_f * (n_f - 1.0));
    let c1 = b1 - 1.0 / a1;
    let c2 = b2 - (n_f + 2.0) / (a1 * n_f) + a2 / (a1 * a1);
    let e1 = c1 / a1;
    let e2 = c2 / (a1 * a1 + a2);
    let s = theta_w * a1;
    let var = e1 * s + e2 * s * (s - 1.0);
    if var <= 0.0 {
        return 0.0;
    }
    (pi - theta_w) / var.sqrt()
}

pub fn watterson_theta(segregating_sites: usize, n: usize, sequence_length: usize) -> f64 {
    let a1: f64 = (1..n).map(|i| 1.0 / i as f64).sum();
    segregating_sites as f64 / (a1 * sequence_length as f64)
}

pub fn nucleotide_diversity(differences: &[f64], n_sequences: usize) -> f64 {
    let pairs = n_sequences * (n_sequences - 1) / 2;
    if pairs == 0 {
        return 0.0;
    }
    let total: f64 = differences.iter().sum();
    total / pairs as f64
}

pub fn ewens_sampling_formula(n: usize, theta: f64) -> f64 {
    let mut result = 1.0;
    for i in 0..n {
        result *= theta / (theta + i as f64);
    }
    result
}

pub fn coalescent_expected_time(n: usize, ne: f64) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    let mut total = 0.0;
    for k in 2..=n {
        let k_f = k as f64;
        total += 4.0 * ne / (k_f * (k_f - 1.0));
    }
    total
}

pub fn mcdonald_kreitman_ratio(dn: f64, ds: f64, pn: f64, ps: f64) -> f64 {
    (dn * ps) / (ds * pn).max(1e-30)
}

pub fn neutrality_index(dn: f64, ds: f64, pn: f64, ps: f64) -> f64 {
    (pn / ps.max(1e-30)) / (dn / ds.max(1e-30)).max(1e-30)
}

pub fn direction_of_selection(ni: f64) -> f64 {
    1.0 - ni
}
