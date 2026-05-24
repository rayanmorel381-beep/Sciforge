use crate::constants::K_B;

pub fn occupation_number(energy_j: f64, fermi_energy_j: f64, t_k: f64) -> f64 {
    1.0 / (((energy_j - fermi_energy_j) / (K_B * t_k)).exp() + 1.0)
}

pub fn fermi_temperature(fermi_energy_j: f64) -> f64 {
    fermi_energy_j / K_B
}

pub fn fermi_energy_3d(electron_density_per_m3: f64, mass_kg: f64) -> f64 {
    let hbar = crate::constants::HBAR;
    let pi = std::f64::consts::PI;
    hbar * hbar / (2.0 * mass_kg)
        * (3.0 * pi * pi * electron_density_per_m3).powf(2.0 / 3.0)
}

pub fn density_of_states_3d(energy_j: f64, mass_kg: f64) -> f64 {
    let hbar = crate::constants::HBAR;
    let pi = std::f64::consts::PI;
    let prefactor = (2.0 * mass_kg).powf(1.5) / (2.0 * pi * pi * hbar.powi(3));
    prefactor * energy_j.max(0.0).sqrt()
}

pub fn sommerfeld_heat_capacity_coefficient(
    fermi_energy_j: f64,
    electron_density_per_m3: f64,
) -> f64 {
    let pi = std::f64::consts::PI;
    pi * pi * electron_density_per_m3 * K_B * K_B / (2.0 * fermi_energy_j)
}
