pub fn boiling_point_elevation(kb: f64, molality: f64, vant_hoff_factor: f64) -> f64 {
    vant_hoff_factor * kb * molality
}

pub fn freezing_point_depression(kf: f64, molality: f64, vant_hoff_factor: f64) -> f64 {
    vant_hoff_factor * kf * molality
}

pub fn osmotic_pressure(molarity: f64, temperature: f64, vant_hoff_factor: f64) -> f64 {
    vant_hoff_factor * molarity * crate::constants::R_GAS * temperature
}

pub fn vapor_pressure_lowering(x_solvent: f64, p0_solvent: f64) -> f64 {
    p0_solvent * (1.0 - x_solvent)
}

pub fn molar_mass_from_ebullioscopy(
    kb: f64,
    mass_solute: f64,
    mass_solvent_kg: f64,
    delta_t: f64,
) -> f64 {
    kb * mass_solute / (mass_solvent_kg * delta_t).max(1e-30)
}

pub fn molar_mass_from_cryoscopy(
    kf: f64,
    mass_solute: f64,
    mass_solvent_kg: f64,
    delta_t: f64,
) -> f64 {
    kf * mass_solute / (mass_solvent_kg * delta_t).max(1e-30)
}
