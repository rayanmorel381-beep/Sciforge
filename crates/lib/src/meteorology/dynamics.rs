use crate::constants::{EARTH_GRAVITY, SPECIFIC_GAS_CONSTANT_DRY_AIR};

pub fn coriolis_parameter(latitude: f64, omega: f64) -> f64 {
    2.0 * omega * latitude.sin()
}

pub fn geostrophic_wind(dp_dx: f64, dp_dy: f64, rho: f64, f: f64) -> (f64, f64) {
    let ug = -1.0 / (rho * f) * dp_dy;
    let vg = 1.0 / (rho * f) * dp_dx;
    (ug, vg)
}

pub fn rossby_number(u: f64, l: f64, f: f64) -> f64 {
    u / (f * l)
}

pub fn rossby_wave_speed(beta: f64, k: f64) -> f64 {
    -beta / (k * k)
}

pub fn thermal_wind(f: f64, delta_t: f64, delta_x: f64, t_mean: f64) -> f64 {
    SPECIFIC_GAS_CONSTANT_DRY_AIR / f * delta_t / (delta_x * t_mean) * (1000.0_f64).ln()
}

pub fn potential_vorticity(f: f64, dtheta_dp: f64) -> f64 {
    -EARTH_GRAVITY * f * dtheta_dp
}

pub fn ekman_depth(nu: f64, f: f64) -> f64 {
    std::f64::consts::PI * (2.0 * nu / f.abs()).sqrt()
}

pub fn richardson_number(n2: f64, du_dz: f64) -> f64 {
    n2 / (du_dz * du_dz).max(1e-30)
}

pub fn rossby_deformation_radius(n: f64, h: f64, f: f64) -> f64 {
    n * h / f.abs()
}

pub fn cyclone_gradient_wind(r: f64, f: f64, dp_dr: f64, rho: f64) -> f64 {
    let discriminant = f * f * r * r / 4.0 + r / rho * dp_dr;
    if discriminant < 0.0 {
        return 0.0;
    }
    -f * r / 2.0 + discriminant.sqrt()
}
