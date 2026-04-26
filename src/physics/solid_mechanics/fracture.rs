pub fn stress_intensity_factor(sigma: f64, a: f64, geometry_factor: f64) -> f64 {
    geometry_factor * sigma * (std::f64::consts::PI * a).sqrt()
}

pub fn griffith_critical_stress(e: f64, gamma: f64, a: f64) -> f64 {
    (2.0 * e * gamma / (std::f64::consts::PI * a)).sqrt()
}

pub fn energy_release_rate(k: f64, e: f64) -> f64 {
    k * k / e
}

pub fn j_integral(energy_release: f64) -> f64 {
    energy_release
}

pub fn crack_tip_plastic_zone(k: f64, sigma_y: f64) -> f64 {
    (k / sigma_y).powi(2) / (2.0 * std::f64::consts::PI)
}

pub fn paris_law(c: f64, delta_k: f64, m: f64) -> f64 {
    c * delta_k.powf(m)
}

pub fn fatigue_life_basquin(sigma_f: f64, sigma_a: f64, b: f64) -> f64 {
    (sigma_a / sigma_f).powf(1.0 / b) / 2.0
}

pub fn fatigue_life_coffin_manson(ef: f64, ea: f64, c: f64) -> f64 {
    (ea / ef).powf(1.0 / c) / 2.0
}

pub fn miners_rule(cycles: &[f64], max_cycles: &[f64]) -> f64 {
    cycles
        .iter()
        .zip(max_cycles.iter())
        .map(|(&n, &nf)| n / nf)
        .sum()
}

pub fn fracture_toughness_plane_strain(kic: f64, sigma_y: f64) -> f64 {
    2.5 * (kic / sigma_y).powi(2)
}

pub fn stress_corrosion_threshold(k_iscc: f64, sigma: f64, a: f64) -> bool {
    let ki = sigma * (std::f64::consts::PI * a).sqrt();
    ki >= k_iscc
}

pub fn crack_opening_displacement(k: f64, sigma_y: f64, e: f64) -> f64 {
    k * k / (sigma_y * e)
}
