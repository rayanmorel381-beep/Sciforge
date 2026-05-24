pub fn hess_law(enthalpies: &[f64], coefficients: &[f64]) -> f64 {
    enthalpies
        .iter()
        .zip(coefficients.iter())
        .map(|(&h, &c)| h * c)
        .sum()
}

pub fn bond_enthalpy(bonds_broken: &[f64], bonds_formed: &[f64]) -> f64 {
    bonds_broken.iter().sum::<f64>() - bonds_formed.iter().sum::<f64>()
}

pub fn born_haber_lattice_energy(
    sublimation: f64,
    ionization: f64,
    dissociation: f64,
    electron_affinity: f64,
    formation: f64,
) -> f64 {
    formation - sublimation - ionization - dissociation / 2.0 + electron_affinity
}

pub fn calorimetry(mass: f64, specific_heat: f64, delta_t: f64) -> f64 {
    mass * specific_heat * delta_t
}

pub fn kirchhoff(delta_h_t1: f64, delta_cp: f64, t1: f64, t2: f64) -> f64 {
    delta_h_t1 + delta_cp * (t2 - t1)
}

pub fn heat_of_combustion(n_co2: f64, n_h2o: f64, hf_compound: f64) -> f64 {
    n_co2 * (-393.5) + n_h2o * (-285.8) - hf_compound
}

pub fn clausius_clapeyron(p1: f64, delta_h_vap: f64, t1: f64, t2: f64) -> f64 {
    p1 * (delta_h_vap / crate::constants::R_GAS * (1.0 / t1 - 1.0 / t2)).exp()
}

pub fn heat_capacity_ratio(cp: f64, cv: f64) -> f64 {
    cp / cv.max(1e-30)
}

pub fn adiabatic_flame_temperature(
    reactant_enthalpy: f64,
    product_enthalpy_base: f64,
    cp_products: f64,
    t_base: f64,
) -> f64 {
    t_base + (reactant_enthalpy - product_enthalpy_base) / cp_products.max(1e-30)
}

pub fn standard_enthalpy_of_reaction(
    products_hf: &[f64],
    products_coeff: &[f64],
    reactants_hf: &[f64],
    reactants_coeff: &[f64],
) -> f64 {
    let sum_p: f64 = products_hf
        .iter()
        .zip(products_coeff.iter())
        .map(|(&h, &c)| h * c)
        .sum();
    let sum_r: f64 = reactants_hf
        .iter()
        .zip(reactants_coeff.iter())
        .map(|(&h, &c)| h * c)
        .sum();
    sum_p - sum_r
}

pub fn bomb_calorimeter(delta_t: f64, c_cal: f64, mass_sample: f64) -> f64 {
    c_cal * delta_t / mass_sample.max(1e-30)
}
