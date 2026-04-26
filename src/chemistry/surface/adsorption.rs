pub fn langmuir_isotherm(theta_max: f64, k: f64, pressure: f64) -> f64 {
    theta_max * k * pressure / (1.0 + k * pressure)
}

pub fn freundlich_isotherm(kf: f64, pressure: f64, n: f64) -> f64 {
    kf * pressure.powf(1.0 / n.max(1e-30))
}

pub fn bet_isotherm(v_mono: f64, c: f64, p: f64, p0: f64) -> f64 {
    let x = p / p0.max(1e-30);
    v_mono * c * x / ((1.0 - x) * (1.0 - x + c * x))
}

pub fn temkin_isotherm(rt_over_b: f64, a: f64, pressure: f64) -> f64 {
    rt_over_b * (a * pressure).max(1e-30).ln()
}

pub fn langmuir_dissociative(k: f64, pressure: f64) -> f64 {
    let kp_sqrt = (k * pressure).sqrt();
    kp_sqrt / (1.0 + kp_sqrt)
}

pub fn bet_surface_area(v_mono: f64, cross_section: f64, avogadro: f64, molar_volume: f64) -> f64 {
    v_mono / molar_volume.max(1e-30) * avogadro * cross_section
}
