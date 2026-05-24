use std::f64::consts::PI;

pub fn radiation_resistance_short_dipole(length_m: f64, wavelength_m: f64) -> f64 {
    20.0 * PI * PI * (length_m / wavelength_m).powi(2)
}

pub fn radiation_resistance_half_wave_dipole() -> f64 {
    73.1
}

pub fn radiation_resistance_small_loop(area_m2: f64, wavelength_m: f64) -> f64 {
    320.0 * PI.powi(4) * (area_m2 / wavelength_m.powi(2)).powi(2)
}

pub fn directivity_short_dipole() -> f64 {
    1.5
}

pub fn directivity_half_wave_dipole() -> f64 {
    1.643
}

pub fn directivity_isotropic_db() -> f64 {
    0.0
}

pub fn directivity_to_dbi(directivity_linear: f64) -> f64 {
    10.0 * directivity_linear.log10()
}

pub fn gain_from_directivity(directivity_linear: f64, efficiency: f64) -> f64 {
    efficiency * directivity_linear
}

pub fn effective_aperture(gain_linear: f64, wavelength_m: f64) -> f64 {
    gain_linear * wavelength_m * wavelength_m / (4.0 * PI)
}

pub fn effective_aperture_parabolic(physical_area_m2: f64, aperture_efficiency: f64) -> f64 {
    aperture_efficiency * physical_area_m2
}

pub fn parabolic_dish_gain(diameter_m: f64, wavelength_m: f64, efficiency: f64) -> f64 {
    efficiency * (PI * diameter_m / wavelength_m).powi(2)
}

pub fn friis_received_power(
    transmitted_power_w: f64,
    gain_tx_linear: f64,
    gain_rx_linear: f64,
    wavelength_m: f64,
    distance_m: f64,
) -> f64 {
    transmitted_power_w * gain_tx_linear * gain_rx_linear
        * (wavelength_m / (4.0 * PI * distance_m)).powi(2)
}

pub fn free_space_path_loss_db(distance_m: f64, frequency_hz: f64) -> f64 {
    let wavelength = crate::constants::C / frequency_hz;
    20.0 * (4.0 * PI * distance_m / wavelength).log10()
}

pub fn beamwidth_uniform_aperture(diameter_m: f64, wavelength_m: f64) -> f64 {
    1.22 * wavelength_m / diameter_m
}

pub fn beamwidth_3db_uniform_aperture(diameter_m: f64, wavelength_m: f64) -> f64 {
    1.02 * wavelength_m / diameter_m
}

pub fn array_factor_uniform_linear(
    n: u32,
    d_m: f64,
    wavelength_m: f64,
    theta_rad: f64,
    phase_progression_rad: f64,
) -> f64 {
    let psi = 2.0 * PI * d_m / wavelength_m * theta_rad.cos() + phase_progression_rad;
    let half = psi / 2.0;
    if half.sin().abs() < 1e-12 {
        n as f64
    } else {
        (n as f64 * half).sin().abs() / half.sin().abs()
    }
}

pub fn beam_steering_phase(
    d_m: f64,
    wavelength_m: f64,
    steering_angle_rad: f64,
) -> f64 {
    -2.0 * PI * d_m / wavelength_m * steering_angle_rad.sin()
}

pub fn yagi_uda_approximate_gain_dbi(num_elements: u32, boom_length_wavelengths: f64) -> f64 {
    let element_term = (num_elements as f64).log10();
    11.5 + 10.0 * boom_length_wavelengths.log10() + element_term
}

pub fn patch_antenna_resonant_length(epsilon_r: f64, frequency_hz: f64) -> f64 {
    let c = crate::constants::C;
    c / (2.0 * frequency_hz * epsilon_r.sqrt())
}

pub fn patch_antenna_effective_permittivity(epsilon_r: f64, width_m: f64, height_m: f64) -> f64 {
    (epsilon_r + 1.0) / 2.0
        + (epsilon_r - 1.0) / 2.0 * (1.0 + 12.0 * height_m / width_m).powf(-0.5)
}

pub fn horn_antenna_gain(aperture_area_m2: f64, wavelength_m: f64, efficiency: f64) -> f64 {
    efficiency * 4.0 * PI * aperture_area_m2 / (wavelength_m * wavelength_m)
}

pub fn antenna_temperature_to_noise_power(t_antenna_k: f64, bandwidth_hz: f64) -> f64 {
    crate::constants::K_B * t_antenna_k * bandwidth_hz
}

pub fn polarization_loss_factor(angle_mismatch_rad: f64) -> f64 {
    angle_mismatch_rad.cos().powi(2)
}
