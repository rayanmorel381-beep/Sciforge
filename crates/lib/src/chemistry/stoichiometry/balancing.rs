pub fn limiting_reagent(moles: &[f64], coefficients: &[f64]) -> usize {
    moles
        .iter()
        .zip(coefficients.iter())
        .enumerate()
        .min_by(|(_, (m1, c1)), (_, (m2, c2))| {
            (*m1 / c1.max(1e-30))
                .partial_cmp(&(*m2 / c2.max(1e-30)))
                .unwrap()
        })
        .map(|(i, _)| i)
        .unwrap_or(0)
}

pub fn theoretical_yield(
    moles_limiting: f64,
    coeff_limiting: f64,
    coeff_product: f64,
    mw_product: f64,
) -> f64 {
    moles_limiting * coeff_product / coeff_limiting.max(1e-30) * mw_product
}

pub fn percent_yield(actual: f64, theoretical: f64) -> f64 {
    actual / theoretical.max(1e-30) * 100.0
}

pub fn excess_reagent(moles_a: f64, coeff_a: f64, moles_b: f64, coeff_b: f64) -> f64 {
    let ratio_a = moles_a / coeff_a.max(1e-30);
    let ratio_b = moles_b / coeff_b.max(1e-30);
    if ratio_a < ratio_b {
        moles_b - moles_a * coeff_b / coeff_a.max(1e-30)
    } else {
        moles_a - moles_b * coeff_a / coeff_b.max(1e-30)
    }
}

pub fn moles_from_mass(mass: f64, molar_mass: f64) -> f64 {
    mass / molar_mass.max(1e-30)
}

pub fn mass_from_moles(moles: f64, molar_mass: f64) -> f64 {
    moles * molar_mass
}

pub fn number_of_particles(moles: f64) -> f64 {
    moles * crate::constants::N_A
}
