use crate::constants::{C, EPSILON_0, MU_0};

pub fn wave_impedance_free_space() -> f64 {
    (MU_0 / EPSILON_0).sqrt()
}

pub fn wave_number(frequency: f64) -> f64 {
    2.0 * std::f64::consts::PI * frequency / C
}

pub fn wavelength(frequency: f64) -> f64 {
    C / frequency
}

pub fn phase_velocity(epsilon_r: f64, mu_r: f64) -> f64 {
    C / (epsilon_r * mu_r).sqrt()
}

pub fn group_velocity_dispersive(v_phase: f64, omega: f64, dv_domega: f64) -> f64 {
    v_phase / (1.0 - (omega / v_phase) * dv_domega)
}

pub fn skin_depth(frequency: f64, conductivity: f64, mu_r: f64) -> f64 {
    let omega = 2.0 * std::f64::consts::PI * frequency;
    (2.0 / (omega * mu_r * MU_0 * conductivity)).sqrt()
}

pub fn fresnel_rs(n1: f64, n2: f64, theta_i: f64) -> f64 {
    let cos_i = theta_i.cos();
    let sin_t_sq = (n1 / n2).powi(2) * theta_i.sin().powi(2);
    if sin_t_sq > 1.0 {
        return 1.0;
    }
    let cos_t = (1.0 - sin_t_sq).sqrt();
    ((n1 * cos_i - n2 * cos_t) / (n1 * cos_i + n2 * cos_t)).powi(2)
}

pub fn fresnel_rp(n1: f64, n2: f64, theta_i: f64) -> f64 {
    let cos_i = theta_i.cos();
    let sin_t_sq = (n1 / n2).powi(2) * theta_i.sin().powi(2);
    if sin_t_sq > 1.0 {
        return 1.0;
    }
    let cos_t = (1.0 - sin_t_sq).sqrt();
    ((n2 * cos_i - n1 * cos_t) / (n2 * cos_i + n1 * cos_t)).powi(2)
}

pub fn brewster_angle(n1: f64, n2: f64) -> f64 {
    (n2 / n1).atan()
}

pub fn critical_angle(n1: f64, n2: f64) -> Option<f64> {
    if n1 <= n2 {
        return None;
    }
    Some((n2 / n1).asin())
}

pub fn snell(n1: f64, theta1: f64, n2: f64) -> f64 {
    let sin_theta2 = n1 * theta1.sin() / n2;
    sin_theta2.asin()
}

pub fn radiation_pressure_absorbed(intensity: f64) -> f64 {
    intensity / C
}

pub fn radiation_pressure_reflected(intensity: f64) -> f64 {
    2.0 * intensity / C
}

pub fn larmor_radiation_power(charge: f64, accel: f64) -> f64 {
    charge.powi(2) * accel.powi(2) / (6.0 * std::f64::consts::PI * EPSILON_0 * C.powi(3))
}

pub fn antenna_radiation_resistance_dipole(length: f64, wavelength: f64) -> f64 {
    let ratio = length / wavelength;
    80.0 * std::f64::consts::PI * std::f64::consts::PI * ratio * ratio
}

pub fn fdtd_1d(ez: &mut [f64], hy: &mut [f64], steps: usize) {
    let n = ez.len();
    for _ in 0..steps {
        for k in 0..n - 1 {
            hy[k] += ez[k + 1] - ez[k];
        }
        for k in 1..n {
            ez[k] += hy[k] - hy[k - 1];
        }
    }
}

pub fn waveguide_cutoff_te(m: u32, n: u32, a: f64, b: f64) -> f64 {
    C / 2.0 * ((m as f64 / a).powi(2) + (n as f64 / b).powi(2)).sqrt()
}
