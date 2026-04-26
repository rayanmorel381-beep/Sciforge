pub fn glen_strain_rate(a: f64, tau: f64, n: f64) -> f64 {
    a * tau.powf(n)
}

pub fn shallow_ice_velocity(a: f64, n: f64, rho: f64, g: f64, alpha: f64, h: f64) -> f64 {
    (2.0 * a / (n + 1.0)) * (rho * g * alpha.sin()).powf(n) * h.powf(n + 1.0)
}

pub fn ice_viscosity(a: f64, tau: f64, n: f64) -> f64 {
    1.0 / (2.0 * a * tau.powf(n - 1.0))
}

pub fn glacial_bed_erosion(kg: f64, vb: f64, l: f64) -> f64 {
    kg * vb.powf(l)
}
