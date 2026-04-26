pub fn lattice_energy_born_lande(
    madelung: f64,
    z_plus: f64,
    z_minus: f64,
    r0: f64,
    born_exponent: f64,
) -> f64 {
    let e2 = 1.389e-9;
    -madelung * z_plus * z_minus * e2 / r0 * (1.0 - 1.0 / born_exponent)
}

pub fn lattice_energy_kapustinskii(
    nu: u32,
    z_plus: f64,
    z_minus: f64,
    r_plus: f64,
    r_minus: f64,
) -> f64 {
    1202.5 * nu as f64 * z_plus * z_minus / (r_plus + r_minus) * (1.0 - 0.345 / (r_plus + r_minus))
}

pub fn pauling_electronegativity_difference(en_a: f64, en_b: f64) -> f64 {
    (en_a - en_b).abs()
}

pub fn percent_ionic_character(en_diff: f64) -> f64 {
    (1.0 - (-0.25 * en_diff * en_diff).exp()) * 100.0
}
