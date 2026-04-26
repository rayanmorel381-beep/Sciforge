pub fn potential_intensity(ck: f64, cd: f64, eta: f64, delta_k: f64) -> f64 {
    (ck / cd * eta * delta_k).sqrt()
}

pub fn accumulated_cyclone_energy(v_kt_series: &[f64]) -> f64 {
    v_kt_series.iter().map(|v| v.powi(2)).sum::<f64>() / 1.0e4
}

pub fn cape(t_parcel: f64, t_env: f64, g: f64, dz: f64) -> f64 {
    g * (t_parcel - t_env) / t_env * dz
}

pub fn rossby_deformation_radius(n: f64, h: f64, f: f64) -> f64 {
    n * h / f
}

pub fn fujita_scale(v: f64) -> u8 {
    if v < 29.0 {
        return 0;
    }
    if v < 38.0 {
        return 0;
    }
    if v < 49.0 {
        return 1;
    }
    if v < 60.0 {
        return 2;
    }
    if v < 74.0 {
        return 3;
    }
    if v < 89.0 {
        return 4;
    }
    5
}
