pub fn substitution_rate(mu: f64, ne: f64, s: f64) -> f64 {
    if s.abs() < 1e-12 {
        return mu;
    }
    let nes = 2.0 * ne * s;
    if nes > 500.0 {
        return mu * 2.0 * nes;
    }
    if nes < -500.0 {
        return 0.0;
    }
    mu * 2.0 * nes / (1.0 - (-2.0 * nes).exp())
}

pub fn dn_ds_ratio(dn: f64, ds: f64) -> f64 {
    if ds < 1e-30 {
        return f64::INFINITY;
    }
    dn / ds
}

pub fn molecular_clock_rate(substitutions: f64, divergence_time: f64) -> f64 {
    substitutions / (2.0 * divergence_time)
}

pub fn coalescent_time_pair(ne: f64) -> f64 {
    2.0 * ne
}

pub fn expected_segregating_sites(theta: f64, n: usize) -> f64 {
    let harmonic: f64 = (1..n).map(|i| 1.0 / i as f64).sum();
    theta * harmonic
}

pub fn watterson_estimator(seg_sites: usize, n: usize) -> f64 {
    let harmonic: f64 = (1..n).map(|i| 1.0 / i as f64).sum();
    seg_sites as f64 / harmonic
}

pub fn mcdonald_kreitman(dn: f64, ds: f64, pn: f64, ps: f64) -> f64 {
    if ds < 1e-30 || ps < 1e-30 {
        return 0.0;
    }
    1.0 - (dn * ps) / (ds * pn)
}
