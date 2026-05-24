use crate::constants::{C, H, HC_EV_NM, N_A};

pub fn quantum_yield(molecules_reacted: f64, photons_absorbed: f64) -> f64 {
    molecules_reacted / photons_absorbed.max(1e-30)
}

pub fn photon_energy(wavelength_nm: f64) -> f64 {
    H * C / (wavelength_nm * 1e-9).max(1e-30)
}

pub fn photon_energy_ev(wavelength_nm: f64) -> f64 {
    HC_EV_NM / wavelength_nm.max(1e-30)
}

pub fn einstein_energy(wavelength_nm: f64) -> f64 {
    photon_energy(wavelength_nm) * N_A
}

pub fn fluorescence_lifetime(rate_radiative: f64, rate_non_radiative: f64) -> f64 {
    1.0 / (rate_radiative + rate_non_radiative).max(1e-30)
}

pub fn fluorescence_quantum_yield(rate_radiative: f64, rate_non_radiative: f64) -> f64 {
    rate_radiative / (rate_radiative + rate_non_radiative).max(1e-30)
}
