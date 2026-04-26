pub fn absorption_coefficient(i0: f64, i: f64, x: f64) -> f64 {
    (i0 / i).ln() / x
}

pub fn intensity_after_absorption(i0: f64, alpha: f64, x: f64) -> f64 {
    i0 * (-alpha * x).exp()
}

pub fn atmospheric_absorption(f: f64, humidity: f64, temperature: f64) -> f64 {
    let f_khz = f / 1000.0;
    let base = 0.01 * f_khz * f_khz;
    let humidity_factor = 1.0 / (humidity / 50.0).max(0.1);
    let temp_factor = (temperature / 293.0).powf(2.5);
    base * humidity_factor * temp_factor
}

pub fn noise_reduction_coefficient(alphas: &[f64]) -> f64 {
    if alphas.is_empty() {
        return 0.0;
    }
    alphas.iter().sum::<f64>() / alphas.len() as f64
}

pub fn sound_transmission_class(tl_values: &[f64]) -> f64 {
    if tl_values.is_empty() {
        return 0.0;
    }
    tl_values.iter().sum::<f64>() / tl_values.len() as f64
}

pub fn mass_law_transmission_loss(f: f64, surface_density: f64) -> f64 {
    20.0 * (std::f64::consts::PI * f * surface_density / (1.21 * 343.0)).log10()
}

pub fn porous_absorber_flow_resistivity(sigma: f64, thickness: f64, f: f64) -> f64 {
    let x = 1000.0 * f / sigma;
    1.0 - (-0.0571 * (sigma * thickness / (1.21 * 343.0))).exp() * x.powf(-0.754)
}

pub fn a_weighting(f: f64) -> f64 {
    let f2 = f * f;
    let num = 12194.0_f64.powi(2) * f2 * f2;
    let den = (f2 + 20.6_f64.powi(2))
        * ((f2 + 107.7_f64.powi(2)) * (f2 + 737.9_f64.powi(2))).sqrt()
        * (f2 + 12194.0_f64.powi(2));
    2.0 + 20.0 * (num / den).log10()
}

pub fn decibel_addition(levels: &[f64]) -> f64 {
    let sum: f64 = levels.iter().map(|&l| 10.0_f64.powf(l / 10.0)).sum();
    10.0 * sum.log10()
}

pub fn room_constant(s: f64, alpha_avg: f64) -> f64 {
    s * alpha_avg / (1.0 - alpha_avg).max(1e-30)
}
