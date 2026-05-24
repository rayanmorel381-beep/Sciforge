pub fn goodman_criterion(sigma_a: f64, sigma_m: f64, sigma_e: f64, sigma_u: f64) -> f64 {
    sigma_a / sigma_e + sigma_m / sigma_u
}

pub fn soderberg_criterion(sigma_a: f64, sigma_m: f64, sigma_e: f64, sigma_y: f64) -> f64 {
    sigma_a / sigma_e + sigma_m / sigma_y
}

pub fn gerber_criterion(sigma_a: f64, sigma_m: f64, sigma_e: f64, sigma_u: f64) -> f64 {
    sigma_a / sigma_e + (sigma_m / sigma_u).powi(2)
}

pub fn morrow_correction(sigma_a: f64, sigma_m: f64, sigma_f_prime: f64) -> f64 {
    sigma_a / (sigma_f_prime - sigma_m)
}

pub fn swt_parameter(sigma_max: f64, epsilon_a: f64, e: f64) -> f64 {
    (sigma_max * epsilon_a * e).max(0.0).sqrt()
}

pub fn walker_correction(sigma_a: f64, sigma_max: f64, gamma: f64) -> f64 {
    sigma_max.powf(1.0 - gamma) * sigma_a.powf(gamma)
}

pub fn sn_curve_basquin_inverse(sigma_a: f64, sigma_f_prime: f64, b: f64) -> f64 {
    0.5 * (sigma_a / sigma_f_prime).powf(1.0 / b)
}

pub fn endurance_factor_marin(
    k_load: f64,
    k_size: f64,
    k_surface: f64,
    k_temperature: f64,
    k_reliability: f64,
    k_misc: f64,
) -> f64 {
    k_load * k_size * k_surface * k_temperature * k_reliability * k_misc
}
