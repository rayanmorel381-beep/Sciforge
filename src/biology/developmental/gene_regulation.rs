pub fn gene_regulatory_network_step(
    expression: &mut [f64],
    interactions: &[Vec<f64>],
    basal_rates: &[f64],
    degradation: &[f64],
    dt: f64,
) {
    let n = expression.len();
    let old = expression.to_vec();
    for i in 0..n {
        let mut input = basal_rates[i];
        for (j, &old_j) in old.iter().enumerate() {
            input += interactions[i][j] * old_j;
        }
        let activation = 1.0 / (1.0 + (-input).exp());
        expression[i] = (old[i] + (activation - degradation[i] * old[i]) * dt).max(0.0);
    }
}

pub fn toggle_switch(a: f64, b: f64, alpha: f64, beta: f64, n: f64) -> (f64, f64) {
    let da = alpha / (1.0 + b.powf(n)) - a;
    let db = beta / (1.0 + a.powf(n)) - b;
    (da, db)
}

pub fn apical_basal_polarity(par3: f64, par6: f64, atypical_pkc: f64, par1: f64) -> f64 {
    let apical = par3 * par6 * atypical_pkc;
    let basal = par1;
    apical / (apical + basal).max(1e-30)
}

pub fn planar_cell_polarity(
    frizzled: f64,
    vang: f64,
    coupling: f64,
    neighbor_fz: f64,
    neighbor_vang: f64,
) -> (f64, f64) {
    let dfz = coupling * (neighbor_vang - frizzled) + 0.1 * (1.0 - frizzled);
    let dvg = coupling * (neighbor_fz - vang) + 0.1 * (1.0 - vang);
    (dfz, dvg)
}

pub fn notch_delta_lateral_inhibition_ode(
    notch: f64,
    delta: f64,
    neighbor_delta: f64,
    beta_n: f64,
    beta_d: f64,
    k: f64,
    n: f64,
) -> (f64, f64) {
    let dn = beta_n * neighbor_delta.powf(n) / (k.powf(n) + neighbor_delta.powf(n)) - notch;
    let dd = beta_d / (1.0 + notch.powf(n) / k.powf(n)) - delta;
    (dn, dd)
}

pub fn induction_competence(
    signal: f64,
    competence_window: f64,
    time: f64,
    window_center: f64,
) -> f64 {
    let competence = (-((time - window_center) / competence_window).powi(2)).exp();
    signal * competence
}

pub fn reaction_diffusion_activator_inhibitor(
    a: f64,
    h: f64,
    da: f64,
    rho_a: f64,
    mu_a: f64,
    dh: f64,
    rho_h: f64,
    mu_h: f64,
    laplacian_a: f64,
    laplacian_h: f64,
) -> (f64, f64) {
    let da_dt = da * laplacian_a + rho_a * a * a / h.max(1e-30) - mu_a * a;
    let dh_dt = dh * laplacian_h + rho_h * a * a - mu_h * h;
    (da_dt, dh_dt)
}

pub fn hox_gene_expression(position: f64, boundaries: &[(f64, f64)]) -> Vec<bool> {
    boundaries
        .iter()
        .map(|&(lo, hi)| position >= lo && position <= hi)
        .collect()
}

pub fn morphogenetic_field_potential(
    cell_position: (f64, f64),
    field_center: (f64, f64),
    field_strength: f64,
    decay: f64,
) -> f64 {
    let dist = ((cell_position.0 - field_center.0).powi(2)
        + (cell_position.1 - field_center.1).powi(2))
    .sqrt();
    field_strength * (-decay * dist).exp()
}
