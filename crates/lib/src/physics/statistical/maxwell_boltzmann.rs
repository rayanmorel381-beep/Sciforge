use crate::constants::K_B;
use std::f64::consts::PI;

pub fn speed_distribution(speed_m_per_s: f64, mass_kg: f64, t_k: f64) -> f64 {
    let factor = (mass_kg / (2.0 * PI * K_B * t_k)).powf(1.5);
    4.0 * PI * speed_m_per_s * speed_m_per_s
        * factor
        * (-mass_kg * speed_m_per_s * speed_m_per_s / (2.0 * K_B * t_k)).exp()
}

pub fn most_probable_speed(mass_kg: f64, t_k: f64) -> f64 {
    (2.0 * K_B * t_k / mass_kg).sqrt()
}

pub fn mean_speed(mass_kg: f64, t_k: f64) -> f64 {
    (8.0 * K_B * t_k / (PI * mass_kg)).sqrt()
}

pub fn rms_speed(mass_kg: f64, t_k: f64) -> f64 {
    (3.0 * K_B * t_k / mass_kg).sqrt()
}

pub fn energy_distribution(energy_j: f64, t_k: f64) -> f64 {
    2.0 * (energy_j / PI).sqrt()
        / (K_B * t_k).powf(1.5)
        * (-energy_j / (K_B * t_k)).exp()
}

pub fn equipartition_energy(degrees_of_freedom: f64, t_k: f64) -> f64 {
    0.5 * degrees_of_freedom * K_B * t_k
}
