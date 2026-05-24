pub fn quorum_sensing_ahl(n: f64, k_prod: f64, k_deg: f64, diffusion: f64) -> f64 {
    k_prod * n / (k_deg + diffusion)
}

pub fn quorum_activation(ahl: f64, threshold: f64, n: f64) -> f64 {
    let an = ahl.powf(n);
    an / (threshold.powf(n) + an)
}

pub fn quorum_bistable(
    ahl0: f64,
    n_cells: f64,
    k_prod: f64,
    k_deg: f64,
    k_auto: f64,
    threshold: f64,
    hill_n: f64,
    dt: f64,
    steps: usize,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut ahl = ahl0;
    result.push(ahl);
    for _ in 0..steps {
        let activation = quorum_activation(ahl, threshold, hill_n);
        let production = n_cells * (k_prod + k_auto * activation);
        let degradation = k_deg * ahl;
        ahl = (ahl + (production - degradation) * dt).max(0.0);
        result.push(ahl);
    }
    result
}

pub fn biofilm_growth(
    b: f64,
    mu: f64,
    k_attach: f64,
    planktonic: f64,
    k_detach: f64,
    k_max: f64,
) -> f64 {
    mu * b * (1.0 - b / k_max) + k_attach * planktonic - k_detach * b
}

pub fn biofilm_simulate(
    b0: f64,
    planktonic0: f64,
    mu: f64,
    k_attach: f64,
    k_detach: f64,
    k_max: f64,
    growth_p: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut b, mut p) = (b0, planktonic0);
    result.push((b, p));
    for _ in 0..steps {
        let db = biofilm_growth(b, mu, k_attach, p, k_detach, k_max);
        let dp = growth_p * p - k_attach * p + k_detach * b;
        b = (b + db * dt).max(0.0);
        p = (p + dp * dt).max(0.0);
        result.push((b, p));
    }
    result
}

pub fn antibiotic_kill(n: f64, mic: f64, conc: f64, k_max: f64, hill: f64) -> f64 {
    let cn = conc.powf(hill);
    -k_max * cn / (mic.powf(hill) + cn) * n
}

pub fn mutation_resistance_probability(mu: f64, n: f64) -> f64 {
    1.0 - (-mu * n).exp()
}
