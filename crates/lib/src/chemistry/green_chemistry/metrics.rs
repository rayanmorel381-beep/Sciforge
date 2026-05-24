pub fn atom_economy(mw_product: f64, mw_all_products: f64) -> f64 {
    mw_product / mw_all_products.max(1e-30) * 100.0
}

pub fn e_factor(mass_waste: f64, mass_product: f64) -> f64 {
    mass_waste / mass_product.max(1e-30)
}

pub fn process_mass_intensity(total_mass_input: f64, mass_product: f64) -> f64 {
    total_mass_input / mass_product.max(1e-30)
}

pub fn reaction_mass_efficiency(mass_product: f64, total_mass_reactants: f64) -> f64 {
    mass_product / total_mass_reactants.max(1e-30) * 100.0
}

pub fn carbon_efficiency(mass_carbon_product: f64, mass_carbon_reactants: f64) -> f64 {
    mass_carbon_product / mass_carbon_reactants.max(1e-30) * 100.0
}

pub fn mass_productivity(mass_product: f64, total_mass_used: f64) -> f64 {
    mass_product / total_mass_used.max(1e-30) * 100.0
}

pub fn solvent_intensity(mass_solvent: f64, mass_product: f64) -> f64 {
    mass_solvent / mass_product.max(1e-30)
}

pub fn water_intensity(mass_water: f64, mass_product: f64) -> f64 {
    mass_water / mass_product.max(1e-30)
}

pub fn effective_mass_yield(mass_product: f64, mass_non_benign: f64) -> f64 {
    mass_product / (mass_product + mass_non_benign).max(1e-30) * 100.0
}

pub fn renewable_feedstock_index(mass_renewable: f64, total_mass: f64) -> f64 {
    mass_renewable / total_mass.max(1e-30) * 100.0
}
