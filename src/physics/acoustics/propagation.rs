pub fn speed_of_sound_gas(gamma: f64, r: f64, t: f64, m: f64) -> f64 {
    (gamma * r * t / m).sqrt()
}

pub fn speed_of_sound_solid(e: f64, rho: f64) -> f64 {
    (e / rho).sqrt()
}

pub fn wavelength(speed: f64, frequency: f64) -> f64 {
    speed / frequency
}

pub fn intensity(power: f64, area: f64) -> f64 {
    power / area.max(1e-30)
}

pub fn intensity_level_db(intensity: f64, i_ref: f64) -> f64 {
    10.0 * (intensity / i_ref).log10()
}

pub fn sound_pressure_level(p: f64, p_ref: f64) -> f64 {
    20.0 * (p / p_ref).log10()
}

pub fn inverse_square_law(i0: f64, r0: f64, r: f64) -> f64 {
    i0 * (r0 / r).powi(2)
}

pub fn acoustic_impedance(rho: f64, c: f64) -> f64 {
    rho * c
}

pub fn transmission_coefficient(z1: f64, z2: f64) -> f64 {
    4.0 * z1 * z2 / (z1 + z2).powi(2)
}

pub fn reflection_coefficient(z1: f64, z2: f64) -> f64 {
    ((z2 - z1) / (z2 + z1)).powi(2)
}

pub fn spherical_spreading(p0: f64, r0: f64, r: f64) -> f64 {
    p0 * r0 / r
}

pub fn plane_wave_pressure(rho: f64, c: f64, v: f64) -> f64 {
    rho * c * v
}
