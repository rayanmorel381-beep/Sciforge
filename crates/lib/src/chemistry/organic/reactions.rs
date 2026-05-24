pub fn sn1_rate(k: f64, substrate: f64) -> f64 {
    k * substrate
}

pub fn sn2_rate(k: f64, substrate: f64, nucleophile: f64) -> f64 {
    k * substrate * nucleophile
}

pub fn e1_rate(k: f64, substrate: f64) -> f64 {
    k * substrate
}

pub fn e2_rate(k: f64, substrate: f64, base: f64) -> f64 {
    k * substrate * base
}

pub fn hammett_equation(rho: f64, sigma: f64, log_k0: f64) -> f64 {
    log_k0 + rho * sigma
}

pub fn hammett_acidity(pka_ref: f64, rho: f64, sigma: f64) -> f64 {
    pka_ref - rho * sigma
}

pub fn taft_equation(rho_star: f64, sigma_star: f64, es: f64, delta: f64) -> f64 {
    rho_star * sigma_star + delta * es
}
