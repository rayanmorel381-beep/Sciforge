pub fn mass_transfer_coefficient_film(d: f64, delta: f64) -> f64 {
    d / delta.max(1e-30)
}

pub fn mass_flux(k: f64, c_bulk: f64, c_surface: f64) -> f64 {
    k * (c_bulk - c_surface)
}

pub fn sherwood_number(k: f64, l: f64, d: f64) -> f64 {
    k * l / d.max(1e-30)
}

pub fn schmidt_number(viscosity: f64, density: f64, d: f64) -> f64 {
    viscosity / (density * d).max(1e-30)
}

pub fn penetration_theory_coefficient(d: f64, t_contact: f64) -> f64 {
    2.0 * (d / (std::f64::consts::PI * t_contact).max(1e-30)).sqrt()
}

pub fn surface_renewal_coefficient(d: f64, s: f64) -> f64 {
    (d * s).sqrt()
}

pub fn two_film_theory_overall(k_g: f64, k_l: f64, henry: f64) -> f64 {
    1.0 / (1.0 / k_g.max(1e-30) + henry / k_l.max(1e-30))
}

pub fn mass_transfer_biot(k_ext: f64, r: f64, d_eff: f64) -> f64 {
    k_ext * r / d_eff.max(1e-30)
}

pub fn overall_mass_transfer_resistance(resistances: &[f64]) -> f64 {
    1.0 / resistances
        .iter()
        .map(|&r| 1.0 / r.max(1e-30))
        .sum::<f64>()
        .max(1e-30)
}

pub fn ntu_mass_transfer(k: f64, a: f64, flow: f64) -> f64 {
    k * a / flow.max(1e-30)
}
