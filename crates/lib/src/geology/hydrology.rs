pub fn manning_velocity(n: f64, rh: f64, s: f64) -> f64 {
    (1.0 / n) * rh.powf(2.0 / 3.0) * s.sqrt()
}

pub fn chezy_velocity(c: f64, rh: f64, s: f64) -> f64 {
    c * (rh * s).sqrt()
}

pub fn froude_number(v: f64, g: f64, d: f64) -> f64 {
    v / (g * d).sqrt()
}

pub fn reynolds_number(v: f64, d: f64, nu: f64) -> f64 {
    v * d / nu
}

pub fn stream_power(rho: f64, g: f64, q: f64, s: f64) -> f64 {
    rho * g * q * s
}

pub fn hjulstrom_erosion_threshold(d_grain: f64) -> f64 {
    if d_grain < 1.0e-5 {
        return 2.0;
    }
    if d_grain < 5.0e-4 {
        return 0.2 * d_grain.powf(-0.6);
    }
    4.7 * d_grain.powf(0.5)
}
