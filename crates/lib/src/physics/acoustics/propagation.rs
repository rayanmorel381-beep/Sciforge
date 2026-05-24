pub fn speed_of_sound_gas(gamma: f64, r: f64, t: f64, m: f64) -> f64 {
    (gamma * r * t / m).sqrt()
}

pub fn speed_of_sound_solid(e: f64, rho: f64) -> f64 {
    (e / rho).sqrt()
}

pub fn speed_of_sound_solid_longitudinal(e: f64, nu: f64, rho: f64) -> f64 {
    let k_plus_43g = e * (1.0 - nu) / ((1.0 + nu) * (1.0 - 2.0 * nu));
    (k_plus_43g / rho).sqrt()
}

pub fn speed_of_sound_solid_shear(e: f64, nu: f64, rho: f64) -> f64 {
    let g = e / (2.0 * (1.0 + nu));
    (g / rho).sqrt()
}

pub fn speed_of_sound_liquid(bulk_modulus: f64, rho: f64) -> f64 {
    (bulk_modulus / rho).sqrt()
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

pub fn intensity_level_db_ref(intensity: f64) -> f64 {
    intensity_level_db(intensity, crate::constants::REFERENCE_SOUND_INTENSITY_W_M2)
}

pub fn sound_pressure_level(p: f64, p_ref: f64) -> f64 {
    20.0 * (p / p_ref).log10()
}

pub fn sound_pressure_level_air(p: f64) -> f64 {
    sound_pressure_level(p, crate::constants::REFERENCE_SOUND_PRESSURE_AIR_PA)
}

pub fn sound_pressure_level_water(p: f64) -> f64 {
    sound_pressure_level(p, crate::constants::REFERENCE_SOUND_PRESSURE_WATER_PA)
}

pub fn sound_power_level_db(power: f64) -> f64 {
    10.0 * (power / crate::constants::REFERENCE_SOUND_POWER_W).log10()
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
