use crate::constants::R_GAS;

pub fn fluvial_erosion_rate(k: f64, p: f64, alpha: f64, vc: f64) -> f64 {
    k * p.powf(1.5) * alpha.tan() * (1.0 - vc)
}

pub fn chemical_weathering_rate(a: f64, ea: f64, t: f64, p: f64) -> f64 {
    a * (-ea / (R_GAS * t)).exp() * p.powf(0.65)
}

pub fn frost_weathering_rate(n_ft: f64, phi: f64) -> f64 {
    0.001 * n_ft * phi
}

pub fn wind_erosion_threshold(rho_p: f64, rho_a: f64, g: f64, d: f64) -> f64 {
    0.1 * ((rho_p - rho_a) * g * d / rho_a).sqrt()
}

pub fn volcanic_explosivity_index(volume_km3: f64) -> u8 {
    if volume_km3 < 1.0e-5 {
        return 0;
    }
    if volume_km3 < 1.0e-4 {
        return 1;
    }
    if volume_km3 < 1.0e-3 {
        return 2;
    }
    if volume_km3 < 1.0e-2 {
        return 3;
    }
    if volume_km3 < 1.0e-1 {
        return 4;
    }
    if volume_km3 < 1.0 {
        return 5;
    }
    if volume_km3 < 10.0 {
        return 6;
    }
    if volume_km3 < 100.0 {
        return 7;
    }
    8
}
