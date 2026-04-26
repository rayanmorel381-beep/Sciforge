//! Mathematical physics: de Broglie wavelength, partition functions,
//! Planck distribution, and quantum-statistical mechanics.

use crate::constants::physics::{C, H, HBAR, K_B};

pub fn de_broglie_wavelength(momentum: f64) -> f64 {
    H / momentum
}

pub fn heisenberg_uncertainty_position(delta_p: f64) -> f64 {
    HBAR / (2.0 * delta_p)
}

pub fn heisenberg_uncertainty_momentum(delta_x: f64) -> f64 {
    HBAR / (2.0 * delta_x)
}

pub fn wkb_tunneling_probability(
    energy: f64,
    potential: f64,
    barrier_width: f64,
    particle_mass: f64,
) -> f64 {
    let kappa = (2.0 * particle_mass * (potential - energy)).sqrt() / HBAR;
    (-2.0 * kappa * barrier_width).exp()
}

pub fn partition_function_harmonic(omega: f64, temperature: f64) -> f64 {
    let x = HBAR * omega / (2.0 * K_B * temperature);
    1.0 / (2.0 * x.sinh())
}

pub fn fermi_dirac_distribution(energy: f64, chemical_potential: f64, temperature: f64) -> f64 {
    1.0 / (((energy - chemical_potential) / (K_B * temperature)).exp() + 1.0)
}

pub fn bose_einstein_distribution(energy: f64, chemical_potential: f64, temperature: f64) -> f64 {
    1.0 / (((energy - chemical_potential) / (K_B * temperature)).exp() - 1.0)
}

pub fn density_of_states_3d_free(energy: f64, volume: f64, mass: f64) -> f64 {
    volume * (2.0 * mass).powf(1.5) * energy.sqrt()
        / (2.0 * std::f64::consts::PI * std::f64::consts::PI * HBAR.powi(3))
}

pub fn fourier_mode_frequency(mode_number: f64, length: f64, wave_speed: f64) -> f64 {
    mode_number * wave_speed / (2.0 * length)
}

pub fn relativistic_energy(rest_mass: f64, momentum: f64) -> f64 {
    ((momentum * C).powi(2) + (rest_mass * C * C).powi(2)).sqrt()
}

pub fn thermal_wavelength(mass: f64, temperature: f64) -> f64 {
    H / (2.0 * std::f64::consts::PI * mass * K_B * temperature).sqrt()
}
