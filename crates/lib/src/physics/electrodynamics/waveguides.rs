use std::f64::consts::PI;

pub fn cutoff_frequency_te_mn(m: u32, n: u32, a_m: f64, b_m: f64) -> f64 {
    let c = crate::constants::C;
    let term = (m as f64 / a_m).powi(2) + (n as f64 / b_m).powi(2);
    0.5 * c * term.sqrt()
}

pub fn cutoff_frequency_tm_mn(m: u32, n: u32, a_m: f64, b_m: f64) -> f64 {
    cutoff_frequency_te_mn(m, n, a_m, b_m)
}

pub fn cutoff_frequency_te10_rectangular(a_m: f64) -> f64 {
    crate::constants::C / (2.0 * a_m)
}

pub fn cutoff_wavelength_rectangular(m: u32, n: u32, a_m: f64, b_m: f64) -> f64 {
    let term = (m as f64 / a_m).powi(2) + (n as f64 / b_m).powi(2);
    2.0 / term.sqrt()
}

pub fn guided_wavelength(free_space_wavelength_m: f64, cutoff_wavelength_m: f64) -> f64 {
    let ratio = (free_space_wavelength_m / cutoff_wavelength_m).powi(2);
    if ratio >= 1.0 {
        f64::INFINITY
    } else {
        free_space_wavelength_m / (1.0 - ratio).sqrt()
    }
}

pub fn phase_velocity_waveguide(
    free_space_wavelength_m: f64,
    cutoff_wavelength_m: f64,
) -> f64 {
    let c = crate::constants::C;
    let ratio = (free_space_wavelength_m / cutoff_wavelength_m).powi(2);
    if ratio >= 1.0 {
        f64::INFINITY
    } else {
        c / (1.0 - ratio).sqrt()
    }
}

pub fn group_velocity_waveguide(
    free_space_wavelength_m: f64,
    cutoff_wavelength_m: f64,
) -> f64 {
    let c = crate::constants::C;
    let ratio = (free_space_wavelength_m / cutoff_wavelength_m).powi(2);
    c * (1.0 - ratio).sqrt()
}

pub fn wave_impedance_te(
    free_space_wavelength_m: f64,
    cutoff_wavelength_m: f64,
) -> f64 {
    let eta_0 = (crate::constants::MU_0 / crate::constants::EPSILON_0).sqrt();
    let ratio = (free_space_wavelength_m / cutoff_wavelength_m).powi(2);
    eta_0 / (1.0 - ratio).sqrt()
}

pub fn wave_impedance_tm(
    free_space_wavelength_m: f64,
    cutoff_wavelength_m: f64,
) -> f64 {
    let eta_0 = (crate::constants::MU_0 / crate::constants::EPSILON_0).sqrt();
    let ratio = (free_space_wavelength_m / cutoff_wavelength_m).powi(2);
    eta_0 * (1.0 - ratio).sqrt()
}

pub fn attenuation_te10_rectangular(
    a_m: f64,
    b_m: f64,
    frequency_hz: f64,
    surface_resistance_ohm: f64,
) -> f64 {
    let fc = crate::constants::C / (2.0 * a_m);
    let f = frequency_hz;
    let eta_0 = (crate::constants::MU_0 / crate::constants::EPSILON_0).sqrt();
    let ratio = (fc / f).powi(2);
    let bracket = 1.0 + 2.0 * b_m / a_m * ratio;
    surface_resistance_ohm / (b_m * eta_0 * (1.0 - ratio).sqrt()) * bracket
}

pub fn surface_resistance(omega_rad_per_s: f64, mu_relative: f64, conductivity_s_per_m: f64) -> f64 {
    let mu = mu_relative * crate::constants::MU_0;
    (omega_rad_per_s * mu / (2.0 * conductivity_s_per_m)).sqrt()
}

pub fn cavity_resonant_frequency_rectangular(
    m: u32,
    n: u32,
    p: u32,
    a_m: f64,
    b_m: f64,
    d_m: f64,
) -> f64 {
    let c = crate::constants::C;
    let term = (m as f64 / a_m).powi(2)
        + (n as f64 / b_m).powi(2)
        + (p as f64 / d_m).powi(2);
    0.5 * c * term.sqrt()
}

pub fn cavity_resonant_frequency_cylindrical_te(
    m: u32,
    n: u32,
    p: u32,
    radius_m: f64,
    length_m: f64,
) -> f64 {
    let chi_prime = match (m, n) {
        (0, 1) => 3.832,
        (1, 1) => 1.841,
        (2, 1) => 3.054,
        (0, 2) => 7.016,
        (1, 2) => 5.331,
        _ => 1.841 + (m as f64) * 1.5 + (n as f64) * 1.0,
    };
    let c = crate::constants::C;
    let term =
        (chi_prime / radius_m).powi(2) + (p as f64 * PI / length_m).powi(2);
    c / (2.0 * PI) * term.sqrt()
}

pub fn quality_factor_cavity(
    stored_energy_j: f64,
    power_loss_w: f64,
    frequency_hz: f64,
) -> f64 {
    2.0 * PI * frequency_hz * stored_energy_j / power_loss_w
}

pub fn quality_factor_loaded(q_unloaded: f64, q_external: f64) -> f64 {
    1.0 / (1.0 / q_unloaded + 1.0 / q_external)
}

pub fn coupling_coefficient(q_unloaded: f64, q_external: f64) -> f64 {
    q_unloaded / q_external
}

pub fn dielectric_filled_waveguide_cutoff(
    cutoff_air_hz: f64,
    epsilon_r: f64,
) -> f64 {
    cutoff_air_hz / epsilon_r.sqrt()
}

pub fn power_handling_capacity(
    breakdown_field_v_per_m: f64,
    a_m: f64,
    b_m: f64,
    cutoff_wavelength_m: f64,
    free_space_wavelength_m: f64,
) -> f64 {
    let eta_0 = (crate::constants::MU_0 / crate::constants::EPSILON_0).sqrt();
    let ratio = (free_space_wavelength_m / cutoff_wavelength_m).powi(2);
    breakdown_field_v_per_m * breakdown_field_v_per_m * a_m * b_m
        * (1.0 - ratio).sqrt()
        / (4.0 * eta_0)
}
