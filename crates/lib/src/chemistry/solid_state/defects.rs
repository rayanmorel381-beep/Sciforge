pub fn schottky_defect_concentration(n_sites: f64, e_formation: f64, t: f64) -> f64 {
    n_sites * (-e_formation / (2.0 * crate::constants::KELVIN_TO_KEV * t)).exp()
}

pub fn frenkel_defect_concentration(
    n_sites: f64,
    n_interstitial: f64,
    e_formation: f64,
    t: f64,
) -> f64 {
    (n_sites * n_interstitial).sqrt()
        * (-e_formation / (2.0 * crate::constants::KELVIN_TO_KEV * t)).exp()
}

pub fn vacancy_diffusion_coefficient(d0: f64, q: f64, t: f64) -> f64 {
    d0 * (-q / (crate::constants::R_GAS * t)).exp()
}

pub fn color_center_absorption(wavelength_nm: f64) -> f64 {
    crate::constants::HC_EV_NM / wavelength_nm.max(1e-30)
}

pub fn kroger_vink_equilibrium(k: f64, oxygen_pressure: f64, exponent: f64) -> f64 {
    k * oxygen_pressure.powf(exponent)
}

pub fn nonstoichiometry_delta(
    mass_change: f64,
    molar_mass_oxygen: f64,
    sample_mass: f64,
    molar_mass_sample: f64,
) -> f64 {
    mass_change / molar_mass_oxygen * molar_mass_sample / sample_mass.max(1e-30)
}

pub fn defect_formation_volume(lattice_param_defect: f64, lattice_param_perfect: f64) -> f64 {
    lattice_param_defect.powi(3) - lattice_param_perfect.powi(3)
}

pub fn burgers_vector_magnitude(a: f64, h: i32, k: i32, l: i32) -> f64 {
    a * ((h * h + k * k + l * l) as f64).sqrt()
}
