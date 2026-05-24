pub fn raoult_law(x_solvent: f64, p0_solvent: f64) -> f64 {
    x_solvent * p0_solvent
}

pub fn henrys_law(kh: f64, partial_pressure: f64) -> f64 {
    kh * partial_pressure
}

pub fn mole_fraction(moles_component: f64, total_moles: f64) -> f64 {
    moles_component / total_moles.max(1e-30)
}

pub fn molality(moles_solute: f64, mass_solvent_kg: f64) -> f64 {
    moles_solute / mass_solvent_kg.max(1e-30)
}

pub fn molarity(moles_solute: f64, volume_liters: f64) -> f64 {
    moles_solute / volume_liters.max(1e-30)
}

pub fn gibbs_duhem_check(x1: f64, d_mu1: f64, x2: f64, d_mu2: f64) -> f64 {
    x1 * d_mu1 + x2 * d_mu2
}

pub fn activity_from_mole_fraction(gamma: f64, x: f64) -> f64 {
    gamma * x
}

pub fn margules_one_suffix(a: f64, x1: f64) -> f64 {
    a * x1 * x1
}
