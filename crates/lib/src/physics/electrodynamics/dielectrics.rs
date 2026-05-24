use crate::constants::EPSILON_0;
use std::f64::consts::PI;

pub fn clausius_mossotti(epsilon_r: f64) -> f64 {
    (epsilon_r - 1.0) / (epsilon_r + 2.0)
}

pub fn epsilon_from_clausius_mossotti(polarizability_m3: f64, density_per_m3: f64) -> f64 {
    let cm = density_per_m3 * polarizability_m3 / 3.0;
    (1.0 + 2.0 * cm) / (1.0 - cm)
}

pub fn lorentz_lorenz_index(polarizability_m3: f64, density_per_m3: f64) -> f64 {
    let factor = density_per_m3 * polarizability_m3 / 3.0;
    ((1.0 + 2.0 * factor) / (1.0 - factor)).sqrt()
}

pub fn debye_relaxation_real(epsilon_inf: f64, epsilon_static: f64, omega: f64, tau_s: f64) -> f64 {
    epsilon_inf + (epsilon_static - epsilon_inf) / (1.0 + (omega * tau_s).powi(2))
}

pub fn debye_relaxation_imag(epsilon_static: f64, epsilon_inf: f64, omega: f64, tau_s: f64) -> f64 {
    (epsilon_static - epsilon_inf) * omega * tau_s / (1.0 + (omega * tau_s).powi(2))
}

pub fn cole_cole_real(
    epsilon_inf: f64,
    epsilon_static: f64,
    omega: f64,
    tau_s: f64,
    alpha: f64,
) -> f64 {
    let x = (omega * tau_s).powf(1.0 - alpha);
    let phase = (1.0 - alpha) * PI / 2.0;
    let denom = 1.0 + 2.0 * x * phase.sin() + x * x;
    let num = 1.0 + x * phase.sin();
    epsilon_inf + (epsilon_static - epsilon_inf) * num / denom
}

pub fn cole_cole_imag(
    epsilon_static: f64,
    epsilon_inf: f64,
    omega: f64,
    tau_s: f64,
    alpha: f64,
) -> f64 {
    let x = (omega * tau_s).powf(1.0 - alpha);
    let phase = (1.0 - alpha) * PI / 2.0;
    let denom = 1.0 + 2.0 * x * phase.sin() + x * x;
    (epsilon_static - epsilon_inf) * x * phase.cos() / denom
}

pub fn polarization(epsilon_r: f64, e_field_v_per_m: f64) -> f64 {
    EPSILON_0 * (epsilon_r - 1.0) * e_field_v_per_m
}

pub fn susceptibility(epsilon_r: f64) -> f64 {
    epsilon_r - 1.0
}

pub fn loss_tangent(epsilon_imag: f64, epsilon_real: f64) -> f64 {
    epsilon_imag / epsilon_real
}

pub fn dielectric_loss_power(
    epsilon_r: f64,
    loss_tangent: f64,
    e_field_v_per_m: f64,
    frequency_hz: f64,
) -> f64 {
    let omega = 2.0 * PI * frequency_hz;
    omega * EPSILON_0 * epsilon_r * loss_tangent * e_field_v_per_m * e_field_v_per_m
}

pub fn capacitance_parallel_plate(epsilon_r: f64, area_m2: f64, distance_m: f64) -> f64 {
    EPSILON_0 * epsilon_r * area_m2 / distance_m
}

pub fn capacitance_cylindrical(epsilon_r: f64, length_m: f64, r_inner: f64, r_outer: f64) -> f64 {
    2.0 * PI * EPSILON_0 * epsilon_r * length_m / (r_outer / r_inner).ln()
}

pub fn capacitance_spherical(epsilon_r: f64, r_inner: f64, r_outer: f64) -> f64 {
    4.0 * PI * EPSILON_0 * epsilon_r * r_inner * r_outer / (r_outer - r_inner)
}

pub fn energy_density_dielectric(epsilon_r: f64, e_field_v_per_m: f64) -> f64 {
    0.5 * EPSILON_0 * epsilon_r * e_field_v_per_m * e_field_v_per_m
}

pub fn langevin_function(x: f64) -> f64 {
    if x.abs() < 1e-6 {
        x / 3.0
    } else {
        1.0 / x.tanh() - 1.0 / x
    }
}

pub fn debye_orientational_polarization(
    dipole_moment_cm: f64,
    density_per_m3: f64,
    e_field_v_per_m: f64,
    t_k: f64,
) -> f64 {
    let kt = crate::constants::K_B * t_k;
    density_per_m3 * dipole_moment_cm * dipole_moment_cm * e_field_v_per_m / (3.0 * kt)
}
