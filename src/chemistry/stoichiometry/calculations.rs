pub fn oxidation_number_simple(charge: i32, num_atoms: u32) -> f64 {
    charge as f64 / num_atoms.max(1) as f64
}

pub fn equivalent_mass(molar_mass: f64, n_equivalents: f64) -> f64 {
    molar_mass / n_equivalents.max(1e-30)
}

pub fn normality(equivalents: f64, volume_liters: f64) -> f64 {
    equivalents / volume_liters.max(1e-30)
}

pub fn atom_economy(mw_desired_product: f64, mw_all_products: &[f64]) -> f64 {
    mw_desired_product / mw_all_products.iter().sum::<f64>().max(1e-30) * 100.0
}

pub fn empirical_formula_ratio(masses: &[f64], molar_masses: &[f64]) -> Vec<f64> {
    let moles: Vec<f64> = masses
        .iter()
        .zip(molar_masses.iter())
        .map(|(&m, &mm)| m / mm.max(1e-30))
        .collect();
    let min_mol = moles
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min)
        .max(1e-30);
    moles.iter().map(|&m| m / min_mol).collect()
}

pub fn dilution_factor(v_initial: f64, v_final: f64) -> f64 {
    v_final / v_initial.max(1e-30)
}

pub fn stoichiometric_ratio(coeff_a: f64, coeff_b: f64) -> f64 {
    coeff_a / coeff_b.max(1e-30)
}

pub fn mass_percent(mass_solute: f64, mass_solution: f64) -> f64 {
    mass_solute / mass_solution.max(1e-30) * 100.0
}

pub fn ppm_from_mass(mass_solute: f64, mass_solution: f64) -> f64 {
    mass_solute / mass_solution.max(1e-30) * 1e6
}

pub fn molarity_to_molality(molarity: f64, density: f64, molar_mass_solute: f64) -> f64 {
    let mass_solvent_per_liter = density - molarity * molar_mass_solute / 1000.0;
    molarity / (mass_solvent_per_liter / 1000.0).max(1e-30)
}
