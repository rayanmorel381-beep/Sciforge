use crate::constants::{AMU_TO_KG, C};

pub fn wavenumber_to_wavelength(wavenumber_cm: f64) -> f64 {
    1.0e4 / wavenumber_cm.max(1e-30)
}

pub fn wavelength_to_wavenumber(wavelength_um: f64) -> f64 {
    1.0e4 / wavelength_um.max(1e-30)
}

pub fn wavenumber_to_frequency(wavenumber_cm: f64) -> f64 {
    wavenumber_cm * (C * 1e2)
}

pub fn force_constant_from_frequency(wavenumber: f64, reduced_mass_amu: f64) -> f64 {
    let c_cm = C * 1e2;
    let mu = reduced_mass_amu * AMU_TO_KG;
    let nu = wavenumber * c_cm;
    4.0 * std::f64::consts::PI.powi(2) * nu * nu * mu
}

pub fn reduced_mass(m1: f64, m2: f64) -> f64 {
    m1 * m2 / (m1 + m2).max(1e-30)
}

pub fn ir_intensity_ratio(absorbance_sample: f64, absorbance_reference: f64) -> f64 {
    absorbance_sample / absorbance_reference.max(1e-30)
}
