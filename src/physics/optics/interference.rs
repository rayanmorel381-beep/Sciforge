pub fn two_beam_intensity(i1: f64, i2: f64, delta_phase: f64) -> f64 {
    i1 + i2 + 2.0 * (i1 * i2).sqrt() * delta_phase.cos()
}

pub fn thin_film_phase_shift(n: f64, thickness: f64, wavelength: f64, theta: f64) -> f64 {
    4.0 * std::f64::consts::PI * n * thickness * theta.cos() / wavelength
}

pub fn constructive_condition(optical_path_diff: f64, wavelength: f64) -> bool {
    let m = (optical_path_diff / wavelength).round();
    (optical_path_diff - m * wavelength).abs() < wavelength * 0.01
}

pub fn fringe_spacing(wavelength: f64, d: f64, l: f64) -> f64 {
    wavelength * l / d
}

pub fn visibility(i_max: f64, i_min: f64) -> f64 {
    (i_max - i_min) / (i_max + i_min)
}

pub fn coherence_length(wavelength: f64, delta_wavelength: f64) -> f64 {
    wavelength * wavelength / delta_wavelength
}

pub fn coherence_time(delta_nu: f64) -> f64 {
    1.0 / delta_nu
}

pub fn fabry_perot_transmittance(r: f64, delta: f64) -> f64 {
    let f = 4.0 * r / ((1.0 - r) * (1.0 - r));
    1.0 / (1.0 + f * (delta / 2.0).sin().powi(2))
}

pub fn fabry_perot_finesse(r: f64) -> f64 {
    std::f64::consts::PI * r.sqrt() / (1.0 - r)
}

pub fn free_spectral_range(d: f64, n: f64) -> f64 {
    3e8 / (2.0 * n * d)
}

pub fn michelson_path_difference(mirror_displacement: f64) -> f64 {
    2.0 * mirror_displacement
}

pub fn newton_ring_radius(m: u32, wavelength: f64, r: f64) -> f64 {
    (m as f64 * wavelength * r).sqrt()
}
