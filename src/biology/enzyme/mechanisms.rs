pub fn enzyme_kinetics_solve(
    s0: f64,
    e0: f64,
    k1: f64,
    k_1: f64,
    k2: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut s = s0;
    let mut es = 0.0;
    let mut p = 0.0;
    let e_total = e0;
    result.push((s, es, p));
    for _ in 0..steps {
        let e = e_total - es;
        let ds = -k1 * e * s + k_1 * es;
        let des = k1 * e * s - k_1 * es - k2 * es;
        let dp = k2 * es;
        s += ds * dt;
        es += des * dt;
        p += dp * dt;
        s = s.max(0.0);
        es = es.max(0.0);
        result.push((s, es, p));
    }
    result
}

pub fn ping_pong(a: f64, b: f64, vmax: f64, ka: f64, kb: f64) -> f64 {
    vmax * a * b / (ka * b + kb * a + a * b)
}

pub fn ordered_bi_bi(a: f64, b: f64, vmax: f64, ka: f64, kb: f64, kia: f64) -> f64 {
    vmax * a * b / (kia * kb + kb * a + ka * b + a * b)
}

pub fn random_bi_bi(a: f64, b: f64, vmax: f64, ka: f64, kb: f64, kia: f64, kib: f64) -> f64 {
    vmax * a * b / (kia * kb + kib * a + ka * b + a * b)
}

pub fn substrate_inhibition(s: f64, vmax: f64, km: f64, ki: f64) -> f64 {
    vmax * s / (km + s + s * s / ki)
}

pub fn allosteric_enzyme(s: f64, vmax: f64, k05: f64, n_hill: f64) -> f64 {
    let sn = s.powf(n_hill);
    vmax * sn / (k05.powf(n_hill) + sn)
}

pub fn covalent_modification_cycle(
    substrate: f64,
    kinase_vmax: f64,
    kinase_km: f64,
    phosphatase_vmax: f64,
    phosphatase_km: f64,
) -> f64 {
    let total = substrate;
    total * (kinase_vmax / kinase_km)
        / (kinase_vmax / kinase_km + phosphatase_vmax / phosphatase_km)
}

pub fn enzyme_activation_energy(k_cat: f64, temperature: f64) -> f64 {
    use crate::constants::{H, K_B, R_GAS};
    R_GAS * temperature * ((k_cat * H) / (K_B * temperature)).ln()
}

pub fn suicide_inhibition(e0: f64, inhibitor: f64, ki: f64, kinact: f64, t: f64) -> f64 {
    let kobs = kinact * inhibitor / (ki + inhibitor);
    e0 * (-kobs * t).exp()
}

pub fn enzyme_cooperativity(substrate: f64, vmax: f64, s05: &[f64], weights: &[f64]) -> f64 {
    let n = s05.len().min(weights.len());
    let mut rate = 0.0;
    for i in 0..n {
        rate += weights[i] * vmax * substrate / (s05[i] + substrate);
    }
    rate
}
