use crate::constants::{K_B, N_A, R_GAS};

pub fn bragg_angle(d: f64, wavelength: f64, n: i32) -> f64 {
    let sin_theta = n as f64 * wavelength / (2.0 * d);
    if sin_theta.abs() > 1.0 {
        return f64::NAN;
    }
    sin_theta.asin()
}

pub fn d_spacing_cubic(a: f64, h: i32, k: i32, l: i32) -> f64 {
    a / ((h * h + k * k + l * l) as f64).sqrt()
}

pub fn miller_planar_density_cubic(a: f64, atoms_per_plane: f64, h: i32, k: i32, l: i32) -> f64 {
    let d = d_spacing_cubic(a, h, k, l);
    atoms_per_plane / (d * d)
}

pub fn packing_fraction_bcc() -> f64 {
    std::f64::consts::PI * 3.0_f64.sqrt() / 8.0
}

pub fn packing_fraction_fcc() -> f64 {
    std::f64::consts::PI * 2.0_f64.sqrt() / 6.0
}

pub fn lattice_parameter_from_density(m: f64, z: f64, rho: f64) -> f64 {
    (z * m / (N_A * rho)).powf(1.0 / 3.0)
}

pub fn structure_factor_bcc(h: i32, k: i32, l: i32) -> f64 {
    if (h + k + l) % 2 == 0 { 2.0 } else { 0.0 }
}

pub fn structure_factor_fcc(h: i32, k: i32, l: i32) -> f64 {
    if h % 2 == k % 2 && k % 2 == l % 2 {
        4.0
    } else {
        0.0
    }
}

pub fn debye_temperature(theta_d: f64, t: f64) -> f64 {
    theta_d / t
}

pub fn specific_heat_debye(t: f64, theta_d: f64) -> f64 {
    let x = theta_d / t;
    if x < 0.01 {
        return 3.0 * R_GAS;
    }
    3.0 * R_GAS * 4.0 / x.powi(3) * (x * x * x.exp() / (x.exp() - 1.0).powi(2))
}

pub fn vacancy_concentration(ev: f64, t: f64) -> f64 {
    (-ev / (K_B * t)).exp()
}
