pub fn p_wave_velocity(k: f64, g: f64, rho: f64) -> f64 {
    ((k + 4.0 * g / 3.0) / rho).sqrt()
}

pub fn s_wave_velocity(g: f64, rho: f64) -> f64 {
    (g / rho).sqrt()
}

pub fn richter_magnitude(amplitude: f64, distance_km: f64) -> f64 {
    amplitude.log10() + 3.0 * distance_km.log10() - 2.92
}

pub fn moment_magnitude(seismic_moment: f64) -> f64 {
    (seismic_moment.log10() - 9.1) / 1.5
}

pub fn seismic_moment(mw: f64) -> f64 {
    10.0_f64.powf(1.5 * mw + 9.1)
}

pub fn epicenter_distance(vp: f64, vs: f64, ts_tp: f64) -> f64 {
    ts_tp * vp * vs / (vp - vs)
}

pub fn travel_time(distance: f64, velocity: f64) -> f64 {
    distance / velocity
}

pub fn snell_seismic(v1: f64, theta1: f64, v2: f64) -> f64 {
    let sin2 = v2 * theta1.sin() / v1;
    if sin2.abs() > 1.0 {
        return std::f64::consts::FRAC_PI_2;
    }
    sin2.asin()
}

pub fn gutenberg_richter(a: f64, b: f64, magnitude: f64) -> f64 {
    10.0_f64.powf(a - b * magnitude)
}

pub fn omori_aftershock(k: f64, c: f64, p: f64, t: f64) -> f64 {
    k / (t + c).powf(p)
}

pub fn seismic_energy(magnitude: f64) -> f64 {
    10.0_f64.powf(1.5 * magnitude + 4.8)
}

pub fn peak_ground_acceleration(a: f64, b: f64, magnitude: f64, distance: f64) -> f64 {
    a * (b * magnitude).exp() / distance
}
