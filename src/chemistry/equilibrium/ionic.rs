pub fn common_ion_effect(ksp: f64, common_ion_conc: f64, stoich_coeff: f64) -> f64 {
    (ksp / common_ion_conc.powf(stoich_coeff)).max(0.0)
}

pub fn buffer_capacity(ca: f64, cb: f64, ka: f64, h: f64) -> f64 {
    let ratio = ka * h / (ka + h).powi(2);
    2.303 * (h + 1e-14 / h + ca * ratio + cb * ratio)
}

pub fn ph_weak_acid(ka: f64, c: f64) -> f64 {
    let h = (ka * c).sqrt();
    -h.log10()
}

pub fn ph_buffer(ka: f64, acid: f64, base: f64) -> f64 {
    let pka = -ka.log10();
    pka + (base / acid).log10()
}

pub fn solubility_product(ion_concentrations: &[(f64, f64)]) -> f64 {
    ion_concentrations.iter().map(|&(c, n)| c.powf(n)).product()
}

pub fn distribution_coefficient(c_organic: f64, c_aqueous: f64) -> f64 {
    c_organic / c_aqueous.max(1e-30)
}

pub fn solubility_from_ksp(ksp: f64, cation_stoich: f64, anion_stoich: f64) -> f64 {
    let total = cation_stoich + anion_stoich;
    let coeff = cation_stoich.powf(cation_stoich) * anion_stoich.powf(anion_stoich);
    (ksp / coeff).powf(1.0 / total)
}

pub fn formation_constant(product_conc: f64, metal_conc: f64, ligand_conc: f64, n: f64) -> f64 {
    product_conc / (metal_conc * ligand_conc.powf(n)).max(1e-30)
}

pub fn conditional_formation_constant(kf: f64, alpha_y: f64) -> f64 {
    kf * alpha_y
}
