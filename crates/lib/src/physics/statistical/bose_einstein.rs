use crate::constants::K_B;

pub fn occupation_number(energy_j: f64, chemical_potential_j: f64, t_k: f64) -> f64 {
    1.0 / (((energy_j - chemical_potential_j) / (K_B * t_k)).exp() - 1.0)
}

pub fn bec_critical_temperature(particle_density_per_m3: f64, mass_kg: f64) -> f64 {
    let hbar = crate::constants::HBAR;
    let pi = std::f64::consts::PI;
    let zeta_three_halves = 2.612_375_348_685_488;
    2.0 * pi * hbar * hbar / (mass_kg * K_B)
        * (particle_density_per_m3 / zeta_three_halves).powf(2.0 / 3.0)
}

pub fn condensate_fraction(t_k: f64, t_c_k: f64) -> f64 {
    if t_k >= t_c_k {
        0.0
    } else {
        1.0 - (t_k / t_c_k).powf(1.5)
    }
}

pub fn planck_spectral_radiance(frequency_hz: f64, t_k: f64) -> f64 {
    let h = crate::constants::H;
    let c = crate::constants::C;
    let exponent = h * frequency_hz / (K_B * t_k);
    2.0 * h * frequency_hz.powi(3) / (c * c) / (exponent.exp() - 1.0)
}
