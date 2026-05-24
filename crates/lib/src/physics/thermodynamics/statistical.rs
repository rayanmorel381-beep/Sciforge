use crate::constants::{HBAR, K_B};

pub fn boltzmann_distribution(energy: f64, temperature: f64) -> f64 {
    (-energy / (K_B * temperature)).exp()
}

pub fn partition_function_discrete(energies: &[f64], temperature: f64) -> f64 {
    energies
        .iter()
        .map(|&e| (-e / (K_B * temperature)).exp())
        .sum()
}

pub fn partition_function_harmonic(omega: f64, temperature: f64) -> f64 {
    let x = HBAR * omega / (K_B * temperature);
    1.0 / (2.0 * (x / 2.0).sinh())
}

pub fn mean_energy_canonical(energies: &[f64], temperature: f64) -> f64 {
    let z = partition_function_discrete(energies, temperature);
    energies
        .iter()
        .map(|&e| e * (-e / (K_B * temperature)).exp())
        .sum::<f64>()
        / z
}

pub fn entropy_canonical(energies: &[f64], temperature: f64) -> f64 {
    let z = partition_function_discrete(energies, temperature);
    let mean_e = mean_energy_canonical(energies, temperature);
    mean_e / temperature + K_B * z.ln()
}

pub fn fermi_dirac(energy: f64, mu: f64, temperature: f64) -> f64 {
    1.0 / (((energy - mu) / (K_B * temperature)).exp() + 1.0)
}

pub fn bose_einstein(energy: f64, mu: f64, temperature: f64) -> f64 {
    1.0 / (((energy - mu) / (K_B * temperature)).exp() - 1.0)
}

pub fn planck_radiation(frequency: f64, temperature: f64) -> f64 {
    use crate::constants::{C, H};
    let x = H * frequency / (K_B * temperature);
    (2.0 * H * frequency.powi(3) / (C * C)) / (x.exp() - 1.0)
}

pub fn planck_radiation_wavelength(wavelength: f64, temperature: f64) -> f64 {
    use crate::constants::{C, H};
    let x = H * C / (wavelength * K_B * temperature);
    (2.0 * H * C * C / wavelength.powi(5)) / (x.exp() - 1.0)
}

pub fn wien_displacement(temperature: f64) -> f64 {
    2.897_771_955e-3 / temperature
}

pub fn stefan_boltzmann_power(temperature: f64, area: f64) -> f64 {
    use crate::constants::SIGMA_SB;
    SIGMA_SB * area * temperature.powi(4)
}

pub fn debye_heat_capacity(temperature: f64, debye_temp: f64, n_atoms: f64) -> f64 {
    let x = debye_temp / temperature;
    let n_points = 100;
    let dx_int = x / n_points as f64;
    let mut integral = 0.0;
    for i in 0..n_points {
        let t = (i as f64 + 0.5) * dx_int;
        if t > 0.0 {
            let et = t.exp();
            let f = t.powi(4) * et / ((et - 1.0) * (et - 1.0));
            integral += f * dx_int;
        }
    }
    9.0 * n_atoms * K_B * (temperature / debye_temp).powi(3) * integral
}

pub fn einstein_heat_capacity(temperature: f64, einstein_temp: f64, n_atoms: f64) -> f64 {
    let x = einstein_temp / temperature;
    let ex = x.exp();
    3.0 * n_atoms * K_B * x * x * ex / ((ex - 1.0) * (ex - 1.0))
}

pub fn sackur_tetrode(n_moles: f64, temperature: f64, volume: f64, mass: f64) -> f64 {
    use crate::constants::N_A;
    let n = n_moles * N_A;
    let thermal_wl = (2.0 * std::f64::consts::PI * HBAR * HBAR / (mass * K_B * temperature)).sqrt();
    n * K_B * (volume / (n * thermal_wl.powi(3)) + 2.5).ln()
}

pub fn fermi_energy(mass: f64, number_density: f64) -> f64 {
    let pi = std::f64::consts::PI;
    (HBAR * HBAR / (2.0 * mass)) * (3.0 * pi * pi * number_density).powf(2.0 / 3.0)
}
