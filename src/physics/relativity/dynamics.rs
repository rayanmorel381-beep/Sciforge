use crate::constants::C;

pub fn relativistic_momentum(mass: f64, v: f64) -> f64 {
    mass * v * super::lorentz::gamma_factor(v)
}

pub fn relativistic_energy(mass: f64, v: f64) -> f64 {
    mass * C * C * super::lorentz::gamma_factor(v)
}

pub fn rest_energy(mass: f64) -> f64 {
    mass * C * C
}

pub fn kinetic_energy_relativistic(mass: f64, v: f64) -> f64 {
    relativistic_energy(mass, v) - rest_energy(mass)
}

pub fn energy_momentum_relation(mass: f64, momentum: f64) -> f64 {
    ((momentum * C).powi(2) + (mass * C * C).powi(2)).sqrt()
}

pub fn four_momentum(mass: f64, v: [f64; 3]) -> [f64; 4] {
    let v_mag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let g = super::lorentz::gamma_factor(v_mag);
    [
        g * mass * C,
        g * mass * v[0],
        g * mass * v[1],
        g * mass * v[2],
    ]
}

pub fn invariant_mass_two_body(p1: [f64; 4], p2: [f64; 4]) -> f64 {
    let e = p1[0] + p2[0];
    let px = p1[1] + p2[1];
    let py = p1[2] + p2[2];
    let pz = p1[3] + p2[3];
    let s = e * e - (px * px + py * py + pz * pz) * C * C;
    if s > 0.0 { s.sqrt() / C } else { 0.0 }
}

pub fn mandelstam_s(p1: [f64; 4], p2: [f64; 4]) -> f64 {
    let e = p1[0] + p2[0];
    let px = p1[1] + p2[1];
    let py = p1[2] + p2[2];
    let pz = p1[3] + p2[3];
    e * e - (px * px + py * py + pz * pz) * C * C
}

pub fn compton_wavelength_shift(angle: f64) -> f64 {
    use crate::constants::{ELECTRON_MASS_KG, H};
    (H / (ELECTRON_MASS_KG * C)) * (1.0 - angle.cos())
}

pub fn relativistic_kinetic_energy_from_gamma(mass: f64, gamma: f64) -> f64 {
    (gamma - 1.0) * mass * C * C
}

pub fn threshold_energy(m_target: f64, m_products_sum: f64) -> f64 {
    let m_target_e = m_target * C * C;
    (m_products_sum * C * C).powi(2) / (2.0 * m_target_e) - m_target_e / 2.0
}

pub fn synchrotron_power(charge: f64, mass: f64, gamma: f64, radius: f64) -> f64 {
    use crate::constants::EPSILON_0;
    let c3 = C * C * C;
    charge.powi(2) * c3 * gamma.powi(4)
        / (6.0 * std::f64::consts::PI * EPSILON_0 * radius * radius)
        / (mass * C * C / (charge * C)).powi(2)
        * charge.powi(2)
        / (6.0 * std::f64::consts::PI * EPSILON_0)
        * gamma.powi(4)
        / (radius * radius)
}

pub fn bremsstrahlung_power_classical(charge: f64, accel: f64) -> f64 {
    use crate::constants::EPSILON_0;
    charge.powi(2) * accel.powi(2) / (6.0 * std::f64::consts::PI * EPSILON_0 * C * C * C)
}
