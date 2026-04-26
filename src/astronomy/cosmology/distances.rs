use std::f64::consts::PI;

use super::expansion::{e_z, e_z_lcdm, e_z_w0wa, e_z_wcdm, hubble_time_gyr, quad};
use crate::constants::{C_KM_S, CMB_TEMPERATURE, MPC_IN_KM, SEC_PER_GYR};

pub fn comoving_distance_from_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    let d_h = C_KM_S / h0;
    d_h * quad(0.0, z, 1000, |zi| 1.0 / e_z_lcdm(omega_m, zi))
}

pub fn luminosity_distance_from_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    (1.0 + z) * comoving_distance_from_z(h0, omega_m, z)
}

pub fn angular_diameter_distance_from_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    comoving_distance_from_z(h0, omega_m, z) / (1.0 + z)
}

pub fn distance_modulus_from_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    5.0 * luminosity_distance_from_z(h0, omega_m, z).log10() + 25.0
}

pub fn comoving_distance_general(
    h0: f64,
    omega_m: f64,
    omega_r: f64,
    omega_k: f64,
    omega_de: f64,
    z: f64,
) -> f64 {
    let d_h = C_KM_S / h0;
    d_h * quad(0.0, z, 1000, |zi| {
        1.0 / e_z(omega_m, omega_r, omega_k, omega_de, zi)
    })
}

pub fn transverse_comoving_distance(
    h0: f64,
    omega_m: f64,
    omega_r: f64,
    omega_k: f64,
    omega_de: f64,
    z: f64,
) -> f64 {
    let d_h = C_KM_S / h0;
    let chi = quad(0.0, z, 1000, |zi| {
        1.0 / e_z(omega_m, omega_r, omega_k, omega_de, zi)
    });
    if omega_k.abs() < 1e-10 {
        d_h * chi
    } else if omega_k > 0.0 {
        let sqk = omega_k.sqrt();
        d_h / sqk * (sqk * chi).sinh()
    } else {
        let sqk = (-omega_k).sqrt();
        d_h / sqk * (sqk * chi).sin()
    }
}

pub fn luminosity_distance_general(
    h0: f64,
    omega_m: f64,
    omega_r: f64,
    omega_k: f64,
    omega_de: f64,
    z: f64,
) -> f64 {
    (1.0 + z) * transverse_comoving_distance(h0, omega_m, omega_r, omega_k, omega_de, z)
}

pub fn angular_diameter_distance_general(
    h0: f64,
    omega_m: f64,
    omega_r: f64,
    omega_k: f64,
    omega_de: f64,
    z: f64,
) -> f64 {
    transverse_comoving_distance(h0, omega_m, omega_r, omega_k, omega_de, z) / (1.0 + z)
}

pub fn distance_modulus_general(
    h0: f64,
    omega_m: f64,
    omega_r: f64,
    omega_k: f64,
    omega_de: f64,
    z: f64,
) -> f64 {
    5.0 * luminosity_distance_general(h0, omega_m, omega_r, omega_k, omega_de, z).log10() + 25.0
}

pub fn comoving_distance_wcdm(h0: f64, omega_m: f64, omega_de: f64, w: f64, z: f64) -> f64 {
    let d_h = C_KM_S / h0;
    d_h * quad(0.0, z, 1000, |zi| 1.0 / e_z_wcdm(omega_m, omega_de, w, zi))
}

pub fn luminosity_distance_wcdm(h0: f64, omega_m: f64, omega_de: f64, w: f64, z: f64) -> f64 {
    (1.0 + z) * comoving_distance_wcdm(h0, omega_m, omega_de, w, z)
}

pub fn comoving_distance_w0wa(
    h0: f64,
    omega_m: f64,
    omega_de: f64,
    w0: f64,
    wa: f64,
    z: f64,
) -> f64 {
    let d_h = C_KM_S / h0;
    d_h * quad(0.0, z, 1000, |zi| {
        1.0 / e_z_w0wa(omega_m, omega_de, w0, wa, zi)
    })
}

pub fn luminosity_distance_w0wa(
    h0: f64,
    omega_m: f64,
    omega_de: f64,
    w0: f64,
    wa: f64,
    z: f64,
) -> f64 {
    (1.0 + z) * comoving_distance_w0wa(h0, omega_m, omega_de, w0, wa, z)
}

pub fn luminosity_from_angular_diameter(d_a: f64, z: f64) -> f64 {
    d_a * (1.0 + z) * (1.0 + z)
}

pub fn angular_diameter_from_luminosity(d_l: f64, z: f64) -> f64 {
    d_l / ((1.0 + z) * (1.0 + z))
}

pub fn proper_distance(comoving_d: f64, z: f64) -> f64 {
    comoving_d / (1.0 + z)
}

pub fn lookback_time_from_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    hubble_time_gyr(h0)
        * quad(0.0, z, 1000, |zi| {
            1.0 / ((1.0 + zi) * e_z_lcdm(omega_m, zi))
        })
}

pub fn lookback_time_general(
    h0: f64,
    omega_m: f64,
    omega_r: f64,
    omega_k: f64,
    omega_de: f64,
    z: f64,
) -> f64 {
    hubble_time_gyr(h0)
        * quad(0.0, z, 1000, |zi| {
            1.0 / ((1.0 + zi) * e_z(omega_m, omega_r, omega_k, omega_de, zi))
        })
}

pub fn age_at_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    age_lcdm(h0, omega_m) - lookback_time_from_z(h0, omega_m, z)
}

pub fn age_lcdm(h0: f64, omega_m: f64) -> f64 {
    let t_h = hubble_time_gyr(h0);
    let ol = 1.0 - omega_m;
    if ol.abs() < 1e-10 {
        2.0 / 3.0 * t_h
    } else {
        t_h * 2.0 / (3.0 * ol.sqrt()) * (ol / omega_m).sqrt().asinh()
    }
}

pub fn age_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64) -> f64 {
    lookback_time_general(h0, omega_m, omega_r, omega_k, omega_de, 1100.0)
}

pub fn acceleration_redshift(omega_m: f64) -> f64 {
    let omega_l = 1.0 - omega_m;
    (2.0 * omega_l / omega_m).powf(1.0 / 3.0) - 1.0
}

pub fn matter_radiation_equality_z(omega_m: f64, omega_r: f64) -> f64 {
    omega_m / omega_r - 1.0
}

pub fn particle_horizon(h0: f64, omega_m: f64) -> f64 {
    comoving_distance_from_z(h0, omega_m, 1100.0)
}

pub fn particle_horizon_general(
    h0: f64,
    omega_m: f64,
    omega_r: f64,
    omega_k: f64,
    omega_de: f64,
) -> f64 {
    comoving_distance_general(h0, omega_m, omega_r, omega_k, omega_de, 1100.0)
}

pub fn sound_horizon_eh98(omega_m: f64, omega_b: f64, h0: f64) -> f64 {
    let h = h0 / 100.0;
    let om_h2 = omega_m * h * h;
    let ob_h2 = omega_b * h * h;
    let z_eq = 2.5e4 * om_h2 / (CMB_TEMPERATURE / 2.7).powi(4);
    let z_d = 1291.0 * om_h2.powf(0.251) / (1.0 + 0.659 * om_h2.powf(0.828))
        * (1.0 + b1_eh(om_h2, ob_h2) * ob_h2.powf(b2_eh(om_h2)));
    let r_d = 31500.0 * ob_h2 / (CMB_TEMPERATURE / 2.7).powi(4) / (z_d + 1.0);
    let r_eq = 31500.0 * ob_h2 / (CMB_TEMPERATURE / 2.7).powi(4) / (z_eq + 1.0);
    (2.0 / (3.0 * om_h2.sqrt()))
        * (C_KM_S / h0)
        * (6.0 / r_eq).sqrt()
        * ((1.0 + r_d).sqrt() + (r_d + r_eq).sqrt()).ln()
        / (1.0 + (r_eq).sqrt())
}

fn b1_eh(om_h2: f64, ob_h2: f64) -> f64 {
    0.313
        * om_h2.powf(-0.419)
        * (1.0 + 0.607 * om_h2.powf(0.674))
        * (1.0 + ob_h2.powf(0.238 * om_h2.powf(0.223))).recip()
}
fn b2_eh(om_h2: f64) -> f64 {
    0.238 * om_h2.powf(0.223)
}

pub fn event_horizon(h0: f64, omega_m: f64) -> f64 {
    let d_h = C_KM_S / h0;
    d_h * quad(0.0, 1.0, 2000, |a| {
        if a < 1e-10 {
            return 0.0;
        }
        let z = 1.0 / a - 1.0;
        1.0 / (a * a * e_z_lcdm(omega_m, z))
    })
}

pub fn comoving_distance_km(h0: f64, omega_m: f64, z: f64) -> f64 {
    comoving_distance_from_z(h0, omega_m, z) * MPC_IN_KM
}

pub fn lookback_time_seconds(h0: f64, omega_m: f64, z: f64) -> f64 {
    lookback_time_from_z(h0, omega_m, z) * SEC_PER_GYR
}

pub fn solid_angle_from_area(area_mpc2: f64, h0: f64, omega_m: f64, z: f64) -> f64 {
    let d_a = angular_diameter_distance_from_z(h0, omega_m, z);
    area_mpc2 / (d_a * d_a * PI * PI / (180.0 * 180.0))
}
