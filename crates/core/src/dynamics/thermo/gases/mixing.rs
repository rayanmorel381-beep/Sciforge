use crate::moleculars::Gas;

pub fn mixture_molar_mass_kg_per_mol(gases: &[&Gas], mole_fractions: &[f64]) -> f64 {
    gases.iter().zip(mole_fractions.iter()).map(|(g, &y)| y * g.molar_mass_kg_per_mol).sum()
}

pub fn mixture_cp_j_kgk(gases: &[&Gas], mass_fractions: &[f64]) -> f64 {
    gases.iter().zip(mass_fractions.iter()).map(|(g, &y)| y * g.cp_j_kgk_ref).sum()
}

pub fn mixture_cv_j_kgk(gases: &[&Gas], mass_fractions: &[f64]) -> f64 {
    gases.iter().zip(mass_fractions.iter()).map(|(g, &y)| y * g.cv_j_kgk_ref).sum()
}

pub fn mixture_gamma(gases: &[&Gas], mass_fractions: &[f64]) -> f64 {
    mixture_cp_j_kgk(gases, mass_fractions) / mixture_cv_j_kgk(gases, mass_fractions)
}

pub fn partial_pressure_pa(total_pressure_pa: f64, mole_fraction: f64) -> f64 {
    total_pressure_pa * mole_fraction
}

pub fn mole_fraction_from_mass(gas: &Gas, mass_fraction: f64, mixture_molar_mass: f64) -> f64 {
    mass_fraction * mixture_molar_mass / gas.molar_mass_kg_per_mol
}

pub fn mass_fraction_from_mole(gas: &Gas, mole_fraction: f64, mixture_molar_mass: f64) -> f64 {
    mole_fraction * gas.molar_mass_kg_per_mol / mixture_molar_mass
}
