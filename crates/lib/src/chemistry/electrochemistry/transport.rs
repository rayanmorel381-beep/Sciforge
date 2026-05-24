pub fn conductivity(resistance: f64, cell_constant: f64) -> f64 {
    cell_constant / resistance
}

pub fn molar_conductivity(conductivity: f64, concentration: f64) -> f64 {
    conductivity / concentration.max(1e-30)
}

pub fn kohlrausch(lambda_inf: f64, k: f64, c: f64) -> f64 {
    lambda_inf - k * c.sqrt()
}

pub fn debye_huckel_activity(z: f64, ionic_strength: f64) -> f64 {
    let log_gamma = -0.509 * z * z * ionic_strength.sqrt();
    10.0_f64.powf(log_gamma)
}

pub fn ionic_strength(ions: &[(f64, f64)]) -> f64 {
    0.5 * ions.iter().map(|&(c, z)| c * z * z).sum::<f64>()
}

pub fn transference_number(lambda_ion: f64, lambda_total: f64) -> f64 {
    lambda_ion / lambda_total.max(1e-30)
}

pub fn debye_huckel_extended(z: f64, ionic_strength: f64, a_ion: f64) -> f64 {
    let log_gamma =
        -0.509 * z * z * ionic_strength.sqrt() / (1.0 + 0.328 * a_ion * ionic_strength.sqrt());
    10.0_f64.powf(log_gamma)
}

pub fn onsager_equation(lambda_inf: f64, a: f64, b: f64, c: f64) -> f64 {
    lambda_inf - (a + b * lambda_inf) * c.sqrt()
}

pub fn walden_product(viscosity: f64, molar_conductivity: f64) -> f64 {
    viscosity * molar_conductivity
}

pub fn mobility_from_conductivity(lambda_ion: f64, z: f64) -> f64 {
    lambda_ion / (z.abs() * crate::constants::FARADAY).max(1e-30)
}
