pub fn doppler_approaching(f0: f64, c: f64, v_source: f64) -> f64 {
    f0 * c / (c - v_source)
}

pub fn doppler_receding(f0: f64, c: f64, v_source: f64) -> f64 {
    f0 * c / (c + v_source)
}

pub fn doppler_general(f0: f64, c: f64, v_observer: f64, v_source: f64) -> f64 {
    f0 * (c + v_observer) / (c + v_source)
}

pub fn relativistic_doppler(f0: f64, v: f64, c: f64) -> f64 {
    f0 * ((1.0 + v / c) / (1.0 - v / c)).sqrt()
}

pub fn mach_cone_angle(v: f64, c: f64) -> f64 {
    if v <= c {
        return std::f64::consts::FRAC_PI_2;
    }
    (c / v).asin()
}

pub fn doppler_shift_wavelength(lambda0: f64, v: f64, c: f64) -> f64 {
    lambda0 * (1.0 + v / c)
}

pub fn doppler_radar_velocity(delta_f: f64, f0: f64, c: f64) -> f64 {
    delta_f * c / (2.0 * f0)
}

pub fn sonic_boom_pressure(k: f64, l: f64, d: f64, mach: f64) -> f64 {
    k * l / d * (mach * mach - 1.0).sqrt().recip()
}
