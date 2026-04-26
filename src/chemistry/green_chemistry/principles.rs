pub fn catalyst_turnover_number(moles_product: f64, moles_catalyst: f64) -> f64 {
    moles_product / moles_catalyst.max(1e-30)
}

pub fn catalyst_turnover_frequency(ton: f64, time: f64) -> f64 {
    ton / time.max(1e-30)
}

pub fn energy_efficiency(useful_energy: f64, total_energy: f64) -> f64 {
    useful_energy / total_energy.max(1e-30) * 100.0
}

pub fn selectivity(moles_desired: f64, moles_converted: f64) -> f64 {
    moles_desired / moles_converted.max(1e-30) * 100.0
}

pub fn yield_from_selectivity_conversion(selectivity_frac: f64, conversion_frac: f64) -> f64 {
    selectivity_frac * conversion_frac * 100.0
}

pub fn stoichiometric_factor(actual_mass_reactant: f64, stoichiometric_mass: f64) -> f64 {
    actual_mass_reactant / stoichiometric_mass.max(1e-30)
}

pub fn environmental_quotient(e_factor: f64, unfriendliness: f64) -> f64 {
    e_factor * unfriendliness
}

pub fn mass_index(total_mass_input: f64, total_mass_output: f64) -> f64 {
    total_mass_input / total_mass_output.max(1e-30)
}
