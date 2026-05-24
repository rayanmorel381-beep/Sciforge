pub fn rain_rate_marshall_palmer(z: f64) -> f64 {
    (z / 200.0).powf(1.0 / 1.6)
}

pub fn radar_reflectivity(rain_rate: f64) -> f64 {
    200.0 * rain_rate.powf(1.6)
}

pub fn terminal_velocity_raindrop(diameter_mm: f64) -> f64 {
    9.65 - 10.3 * (-0.6 * diameter_mm).exp()
}

pub fn thornthwaite_pet(t_mean: f64, heat_index: f64, day_length_hours: f64) -> f64 {
    let a =
        6.75e-7 * heat_index.powi(3) - 7.71e-5 * heat_index.powi(2) + 1.79e-2 * heat_index + 0.49;
    16.0 * (day_length_hours / 12.0) * (10.0 * t_mean / heat_index).powf(a)
}

pub fn penman_evaporation(delta: f64, rn: f64, gamma: f64, ea: f64, u: f64) -> f64 {
    let wind_func = 0.27 * (1.0 + u / 100.0);
    (delta * rn + gamma * wind_func * ea) / (delta + gamma)
}

pub fn intensity_duration_frequency(a: f64, b: f64, duration: f64, return_period: f64) -> f64 {
    a * return_period.ln() / (duration + b)
}

pub fn scs_curve_number_runoff(p: f64, cn: f64) -> f64 {
    let s = 25400.0 / cn - 254.0;
    let ia = 0.2 * s;
    if p <= ia {
        return 0.0;
    }
    (p - ia).powi(2) / (p - ia + s)
}

pub fn rational_method_runoff(c: f64, i: f64, a: f64) -> f64 {
    c * i * a
}

pub fn unit_hydrograph_peak(a: f64, tp: f64) -> f64 {
    2.08 * a / tp
}

pub fn antecedent_precipitation_index(prev_api: f64, rainfall: f64, k: f64) -> f64 {
    k * prev_api + rainfall
}
