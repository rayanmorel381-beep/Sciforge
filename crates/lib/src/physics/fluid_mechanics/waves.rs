pub fn shallow_water_speed(g: f64, depth: f64) -> f64 {
    (g * depth).sqrt()
}

pub fn deep_water_speed(g: f64, wavelength: f64) -> f64 {
    (g * wavelength / (2.0 * std::f64::consts::PI)).sqrt()
}

pub fn wave_number(wavelength: f64) -> f64 {
    2.0 * std::f64::consts::PI / wavelength
}

pub fn wave_frequency(period: f64) -> f64 {
    2.0 * std::f64::consts::PI / period
}

pub fn froude_number(v: f64, g: f64, depth: f64) -> f64 {
    v / (g * depth).sqrt()
}

pub fn mach_number(v: f64, c: f64) -> f64 {
    v / c
}

pub fn sound_speed_ideal_gas(gamma: f64, r: f64, t: f64, m: f64) -> f64 {
    (gamma * r * t / m).sqrt()
}

pub fn water_hammer_pressure(rho: f64, c: f64, delta_v: f64) -> f64 {
    rho * c * delta_v.abs()
}

pub fn capillary_number(mu: f64, v: f64, sigma: f64) -> f64 {
    mu * v / sigma
}

pub fn weber_number(rho: f64, v: f64, l: f64, sigma: f64) -> f64 {
    rho * v * v * l / sigma
}

pub fn wave_energy_density(rho: f64, g: f64, amplitude: f64) -> f64 {
    0.5 * rho * g * amplitude * amplitude
}
