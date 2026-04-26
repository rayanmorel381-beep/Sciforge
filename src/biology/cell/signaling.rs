pub fn ligand_receptor_binding(l: f64, r_total: f64, kd: f64) -> f64 {
    r_total * l / (kd + l)
}

pub fn hill_response(signal: f64, k: f64, n: f64) -> f64 {
    let sn = signal.powf(n);
    sn / (k.powf(n) + sn)
}

pub fn bistable_switch(x: f64, k1: f64, k2: f64, n: f64, alpha: f64, beta: f64) -> f64 {
    alpha * x.powf(n) / (k1.powf(n) + x.powf(n)) - beta * x / (k2 + x)
}

pub fn bistable_simulate(
    x0: f64,
    k1: f64,
    k2: f64,
    n: f64,
    alpha: f64,
    beta: f64,
    dt: f64,
    steps: usize,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut x = x0;
    result.push(x);
    for _ in 0..steps {
        let dx = bistable_switch(x, k1, k2, n, alpha, beta);
        x += dx * dt;
        x = x.max(0.0);
        result.push(x);
    }
    result
}

pub fn mapk_cascade(
    raf: f64,
    mek: f64,
    erk: f64,
    signal: f64,
    k_activate: &[f64; 3],
    k_deactivate: &[f64; 3],
) -> (f64, f64, f64) {
    let draf = k_activate[0] * signal * (1.0 - raf) - k_deactivate[0] * raf;
    let dmek = k_activate[1] * raf * (1.0 - mek) - k_deactivate[1] * mek;
    let derk = k_activate[2] * mek * (1.0 - erk) - k_deactivate[2] * erk;
    (draf, dmek, derk)
}

pub fn mapk_simulate(
    raf0: f64,
    mek0: f64,
    erk0: f64,
    signal: f64,
    k_activate: &[f64; 3],
    k_deactivate: &[f64; 3],
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut raf, mut mek, mut erk) = (raf0, mek0, erk0);
    result.push((raf, mek, erk));
    for _ in 0..steps {
        let (dr, dm, de) = mapk_cascade(raf, mek, erk, signal, k_activate, k_deactivate);
        raf = (raf + dr * dt).clamp(0.0, 1.0);
        mek = (mek + dm * dt).clamp(0.0, 1.0);
        erk = (erk + de * dt).clamp(0.0, 1.0);
        result.push((raf, mek, erk));
    }
    result
}

pub fn goldbeter_koshland(v1: f64, v2: f64, j1: f64, j2: f64) -> f64 {
    let b = v1 - v2 + j1 * v2 + j2 * v1;
    let discriminant = b * b + 4.0 * (v2 - v1) * j1 * v2;
    if discriminant < 0.0 {
        return 0.5;
    }
    2.0 * j1 * v2 / (b + discriminant.sqrt())
}

pub fn negative_feedback(output: f64, k_prod: f64, k_deg: f64, k_inh: f64, n: f64) -> f64 {
    k_prod / (1.0 + (output / k_inh).powf(n)) - k_deg * output
}

pub fn positive_feedback(x: f64, basal: f64, vmax: f64, k: f64, n: f64, deg: f64) -> f64 {
    basal + vmax * x.powf(n) / (k.powf(n) + x.powf(n)) - deg * x
}

pub fn receptor_desensitization(
    active: f64,
    ligand: f64,
    kd: f64,
    k_intern: f64,
    k_recycle: f64,
    total: f64,
) -> f64 {
    let binding = total * ligand / (kd + ligand);
    -k_intern * active + k_recycle * (binding - active)
}

pub fn dual_phosphorylation(x: f64, kinase: f64, phosphatase: f64, k1: f64, k2: f64) -> f64 {
    kinase * (1.0 - x) / (k1 + 1.0 - x) - phosphatase * x / (k2 + x)
}

pub fn coherent_feedforward(
    signal: f64,
    x: f64,
    k_sx: f64,
    k_xy: f64,
    k_sy: f64,
    threshold: f64,
) -> f64 {
    let dx_dt = k_sx * signal - x;
    let y_input = k_xy * x + k_sy * signal;
    if y_input > threshold { dx_dt } else { -x }
}

pub fn incoherent_feedforward(signal: f64, x: f64, k_activation: f64, k_repression: f64) -> f64 {
    k_activation * signal / (1.0 + signal) - k_repression * x * signal / (1.0 + signal)
}

pub fn michaelis_menten_cascade(substrate: f64, enzyme: f64, km: f64, vmax: f64) -> f64 {
    vmax * enzyme * substrate / (km + substrate)
}

pub fn scaffold_complex_formation(a: f64, b: f64, scaffold: f64, ka: f64, kb: f64) -> f64 {
    scaffold * (a / (ka + a)) * (b / (kb + b))
}

pub fn crosstalk_inhibition(
    pathway_a: f64,
    pathway_b: f64,
    k_inh_ab: f64,
    k_inh_ba: f64,
) -> (f64, f64) {
    let da = -k_inh_ba * pathway_b * pathway_a;
    let db = -k_inh_ab * pathway_a * pathway_b;
    (da, db)
}
