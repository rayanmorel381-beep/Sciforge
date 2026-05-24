pub fn cstr_volume(f_a0: f64, x: f64, r_a: f64) -> f64 {
    f_a0 * x / (-r_a).max(1e-30)
}

pub fn pfr_conversion_first_order(k: f64, tau: f64) -> f64 {
    1.0 - (-k * tau).exp()
}

pub fn cstr_conversion_first_order(k: f64, tau: f64) -> f64 {
    k * tau / (1.0 + k * tau)
}

pub fn batch_time_first_order(k: f64, x: f64) -> f64 {
    -(1.0 - x).max(1e-30).ln() / k.max(1e-30)
}

pub fn batch_time_second_order(k: f64, c0: f64, x: f64) -> f64 {
    x / (k * c0 * (1.0 - x)).max(1e-30)
}

pub fn space_time(volume: f64, volumetric_flow: f64) -> f64 {
    volume / volumetric_flow.max(1e-30)
}

pub fn space_velocity(volumetric_flow: f64, volume: f64) -> f64 {
    volumetric_flow / volume.max(1e-30)
}

pub fn levenspiel_plot_area(fa0_over_ra: &[f64], dx: f64) -> f64 {
    fa0_over_ra.iter().sum::<f64>() * dx
}

pub fn cstr_series_conversion(k: f64, tau_each: f64, n: u32) -> f64 {
    1.0 - 1.0 / (1.0 + k * tau_each).powi(n as i32)
}
