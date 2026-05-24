pub fn coalescent_expected_time(n: usize) -> f64 {
    let mut t = 0.0;
    for k in 2..=n {
        t += 2.0 / (k * (k - 1)) as f64;
    }
    t
}

pub fn watterson_theta(segregating_sites: usize, n: usize) -> f64 {
    let mut a = 0.0;
    for i in 1..n {
        a += 1.0 / i as f64;
    }
    segregating_sites as f64 / a
}

pub fn fst_pairwise(ht: f64, hs: f64) -> f64 {
    if ht < 1e-30 {
        return 0.0;
    }
    (ht - hs) / ht
}

pub fn nucleotide_diversity(sequences: &[&[u8]]) -> f64 {
    let n = sequences.len();
    if n < 2 {
        return 0.0;
    }
    let len = sequences[0].len();
    let mut total_diff = 0.0;
    let mut comparisons = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let diff: usize = (0..len)
                .filter(|&k| sequences[i][k] != sequences[j][k])
                .count();
            total_diff += diff as f64 / len as f64;
            comparisons += 1;
        }
    }
    total_diff / comparisons as f64
}

pub fn tajima_d(pi: f64, theta_w: f64, n: usize, seg_sites: usize) -> f64 {
    if seg_sites == 0 {
        return 0.0;
    }
    let nf = n as f64;
    let mut a1 = 0.0;
    let mut a2 = 0.0;
    for i in 1..n {
        a1 += 1.0 / i as f64;
        a2 += 1.0 / (i * i) as f64;
    }
    let b1 = (nf + 1.0) / (3.0 * (nf - 1.0));
    let b2 = 2.0 * (nf * nf + nf + 3.0) / (9.0 * nf * (nf - 1.0));
    let c1 = b1 - 1.0 / a1;
    let c2 = b2 - (nf + 2.0) / (a1 * nf) + a2 / (a1 * a1);
    let e1 = c1 / a1;
    let e2 = c2 / (a1 * a1 + a2);
    let d = pi - theta_w;
    let var = e1 * seg_sites as f64 + e2 * seg_sites as f64 * (seg_sites as f64 - 1.0);
    if var <= 0.0 {
        return 0.0;
    }
    d / var.sqrt()
}

pub fn nei_distance(pop1_freqs: &[f64], pop2_freqs: &[f64]) -> f64 {
    let jxy: f64 = pop1_freqs
        .iter()
        .zip(pop2_freqs.iter())
        .map(|(&x, &y)| x * y)
        .sum();
    let jx: f64 = pop1_freqs.iter().map(|&x| x * x).sum();
    let jy: f64 = pop2_freqs.iter().map(|&y| y * y).sum();
    let i = jxy / (jx * jy).sqrt();
    if i <= 0.0 {
        return f64::INFINITY;
    }
    -i.ln()
}

pub fn molecular_heterozygosity(freqs: &[f64]) -> f64 {
    1.0 - freqs.iter().map(|&p| p * p).sum::<f64>()
}
