pub fn voigt_modulus(e_f_pa: f64, e_m_pa: f64, vol_frac_f: f64) -> f64 {
    e_f_pa * vol_frac_f + e_m_pa * (1.0 - vol_frac_f)
}

pub fn reuss_modulus(e_f_pa: f64, e_m_pa: f64, vol_frac_f: f64) -> f64 {
    1.0 / (vol_frac_f / e_f_pa + (1.0 - vol_frac_f) / e_m_pa)
}

pub fn hashin_shtrikman_bounds(
    k_f_pa: f64,
    k_m_pa: f64,
    g_f_pa: f64,
    g_m_pa: f64,
    vol_frac_f: f64,
) -> (f64, f64) {
    let vf = vol_frac_f;
    let vm = 1.0 - vf;
    let k_lower = k_m_pa + vf / (1.0 / (k_f_pa - k_m_pa) + 3.0 * vm / (3.0 * k_m_pa + 4.0 * g_m_pa));
    let k_upper = k_f_pa + vm / (1.0 / (k_m_pa - k_f_pa) + 3.0 * vf / (3.0 * k_f_pa + 4.0 * g_f_pa));
    (k_lower, k_upper)
}

pub fn halpin_tsai(e_f_pa: f64, e_m_pa: f64, vol_frac_f: f64, xi: f64) -> f64 {
    let eta = (e_f_pa / e_m_pa - 1.0) / (e_f_pa / e_m_pa + xi);
    e_m_pa * (1.0 + xi * eta * vol_frac_f) / (1.0 - eta * vol_frac_f)
}

pub fn rule_of_mixtures_density(rho_f_kg_m3: f64, rho_m_kg_m3: f64, vol_frac_f: f64) -> f64 {
    rho_f_kg_m3 * vol_frac_f + rho_m_kg_m3 * (1.0 - vol_frac_f)
}

pub fn laminate_abd_q_matrix(e1: f64, e2: f64, nu12: f64, g12: f64) -> [[f64; 3]; 3] {
    let nu21 = nu12 * e2 / e1;
    let denom = 1.0 - nu12 * nu21;
    let q11 = e1 / denom;
    let q22 = e2 / denom;
    let q12 = nu12 * e2 / denom;
    let q66 = g12;
    [[q11, q12, 0.0], [q12, q22, 0.0], [0.0, 0.0, q66]]
}
