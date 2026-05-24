use std::f64::consts::PI;

use crate::constants::{C, C_KM_S, G, MPC_IN_KM, SEC_PER_GYR};

pub(crate) fn quad(a: f64, b: f64, n: usize, f: impl Fn(f64) -> f64) -> f64 {
    let n = if !n.is_multiple_of(2) { n + 1 } else { n };
    let h = (b - a) / n as f64;
    let mut s = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        s += if i % 2 == 0 { 2.0 } else { 4.0 } * f(x);
    }
    s * h / 3.0
}

pub fn hubble_velocity(h0: f64, distance: f64) -> f64 {
    h0 * distance
}
pub fn hubble_distance(h0: f64, velocity: f64) -> f64 {
    velocity / h0
}
pub fn redshift_velocity(v: f64, c: f64) -> f64 {
    v / c
}
pub fn relativistic_redshift(v: f64) -> f64 {
    let beta = v / C;
    ((1.0 + beta) / (1.0 - beta)).sqrt() - 1.0
}

pub fn velocity_from_redshift(z: f64) -> f64 {
    let zp1_sq = (1.0 + z) * (1.0 + z);
    C * (zp1_sq - 1.0) / (zp1_sq + 1.0)
}

pub fn cosmological_redshift(a_emit: f64, a_obs: f64) -> f64 {
    a_obs / a_emit - 1.0
}
pub fn scale_factor(redshift: f64) -> f64 {
    1.0 / (1.0 + redshift)
}
pub fn critical_density(h: f64) -> f64 {
    3.0 * h * h / (8.0 * PI * G)
}

pub fn critical_density_from_h0(h0: f64) -> f64 {
    let h_si = h0 / MPC_IN_KM;
    3.0 * h_si * h_si / (8.0 * PI * G)
}

pub fn friedmann_expansion(h0: f64, omega_m: f64, omega_r: f64, omega_lambda: f64, a: f64) -> f64 {
    h0 * (omega_r / (a * a * a * a) + omega_m / (a * a * a) + omega_lambda).sqrt()
}

pub fn lookback_time(z: f64, h0: f64) -> f64 {
    2.0 / (3.0 * h0) * (1.0 - 1.0 / (1.0 + z).powf(1.5))
}

pub fn luminosity_distance(comoving_distance: f64, z: f64) -> f64 {
    comoving_distance * (1.0 + z)
}

pub fn angular_diameter_distance(comoving_distance: f64, z: f64) -> f64 {
    comoving_distance / (1.0 + z)
}

pub fn cmb_temperature(t0: f64, z: f64) -> f64 {
    t0 * (1.0 + z)
}
pub fn age_of_universe(h0: f64) -> f64 {
    1.0 / h0
}
pub fn e_z(omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64 {
    let zp1 = 1.0 + z;
    (omega_r * zp1.powi(4) + omega_m * zp1.powi(3) + omega_k * zp1 * zp1 + omega_de).sqrt()
}

pub fn e_z_lcdm(omega_m: f64, z: f64) -> f64 {
    let zp1 = 1.0 + z;
    (omega_m * zp1.powi(3) + 1.0 - omega_m).sqrt()
}

pub fn e_z_lcdm_rad(omega_m: f64, omega_r: f64, z: f64) -> f64 {
    let zp1 = 1.0 + z;
    let omega_l = 1.0 - omega_m - omega_r;
    (omega_r * zp1.powi(4) + omega_m * zp1.powi(3) + omega_l).sqrt()
}

pub fn e_z_wcdm(omega_m: f64, omega_de: f64, w: f64, z: f64) -> f64 {
    let zp1 = 1.0 + z;
    (omega_m * zp1.powi(3) + omega_de * zp1.powf(3.0 * (1.0 + w))).sqrt()
}

pub fn e_z_w0wa(omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) -> f64 {
    let zp1 = 1.0 + z;
    let de = omega_de * zp1.powf(3.0 * (1.0 + w0 + wa)) * (-3.0 * wa * z / zp1).exp();
    (omega_m * zp1.powi(3) + de).sqrt()
}

pub fn hubble_at_z(
    h0: f64,
    omega_m: f64,
    omega_r: f64,
    omega_k: f64,
    omega_de: f64,
    z: f64,
) -> f64 {
    h0 * e_z(omega_m, omega_r, omega_k, omega_de, z)
}

pub fn hubble_at_z_lcdm(h0: f64, omega_m: f64, z: f64) -> f64 {
    h0 * e_z_lcdm(omega_m, z)
}

pub fn deceleration_parameter(omega_m: f64, z: f64) -> f64 {
    let zp1 = 1.0 + z;
    let e = e_z_lcdm(omega_m, z);
    let de_dz = (3.0 * omega_m * zp1 * zp1) / (2.0 * e);
    zp1 * de_dz / e - 1.0
}

pub fn deceleration_parameter_today(omega_m: f64) -> f64 {
    omega_m / 2.0 - (1.0 - omega_m)
}

pub fn hubble_distance_mpc(h0: f64) -> f64 {
    C_KM_S / h0
}
pub fn hubble_time_gyr(h0: f64) -> f64 {
    MPC_IN_KM / (h0 * SEC_PER_GYR)
}
pub fn omega_k_from(omega_m: f64, omega_r: f64, omega_de: f64) -> f64 {
    1.0 - omega_m - omega_r - omega_de
}

pub fn little_h(h0: f64) -> f64 {
    h0 / 100.0
}
