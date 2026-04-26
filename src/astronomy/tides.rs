use crate::constants::{G, TIDAL_DISSIPATION_COEFF};

pub fn tidal_potential(m: f64, r: f64, d: f64, k2: f64, theta: f64) -> f64 {
    let p2 = 1.5 * theta.cos().powi(2) - 0.5;
    -(G * m * r.powi(2) / d.powi(3)) * (1.0 + k2) * p2
}

pub fn tidal_bulge_height(a_tidal: f64, r: f64, g: f64, h2: f64) -> f64 {
    h2 * a_tidal * r / g
}

pub fn spring_tide_amplitude(h_moon: f64, h_sun: f64) -> f64 {
    h_moon + h_sun
}

pub fn neap_tide_amplitude(h_moon: f64, h_sun: f64) -> f64 {
    (h_moon - h_sun).abs()
}

pub fn tidal_dissipation_rate(k2: f64, n: f64, m: f64, r: f64, q: f64, d: f64) -> f64 {
    TIDAL_DISSIPATION_COEFF * k2 * n * G * m.powi(2) * r.powi(5) / (q * d.powi(6))
}

pub fn tidal_locking_timescale(omega: f64, a: f64, mu: f64, q: f64, m: f64, r: f64) -> f64 {
    omega * a.powi(6) * mu * q / (6.0 * G * m.powi(2) * r.powi(5))
}
