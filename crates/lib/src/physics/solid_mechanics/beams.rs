use std::f64::consts::PI;

pub fn rectangular_section_i(b: f64, h: f64) -> f64 {
    b * h.powi(3) / 12.0
}

pub fn circular_section_i(d: f64) -> f64 {
    PI * d.powi(4) / 64.0
}

pub fn hollow_circular_i(d_outer: f64, d_inner: f64) -> f64 {
    PI * (d_outer.powi(4) - d_inner.powi(4)) / 64.0
}

pub fn polar_moment_circular(d: f64) -> f64 {
    PI * d.powi(4) / 32.0
}

pub fn euler_bernoulli_deflection(p: f64, x: f64, l: f64, e: f64, i: f64) -> f64 {
    p * x.powi(2) * (3.0 * l - x) / (6.0 * e * i)
}

pub fn timoshenko_deflection(p: f64, l: f64, e: f64, i: f64, g: f64, a_shear: f64) -> f64 {
    p * l.powi(3) / (3.0 * e * i) + p * l / (g * a_shear)
}

pub fn simply_supported_center_deflection(p: f64, l: f64, e: f64, i: f64) -> f64 {
    p * l.powi(3) / (48.0 * e * i)
}

pub fn distributed_load_simply_supported(w: f64, l: f64, e: f64, i: f64) -> f64 {
    5.0 * w * l.powi(4) / (384.0 * e * i)
}

pub fn fixed_fixed_distributed(w: f64, l: f64, e: f64, i: f64) -> f64 {
    w * l.powi(4) / (384.0 * e * i)
}

pub fn cantilever_distributed(w: f64, l: f64, e: f64, i: f64) -> f64 {
    w * l.powi(4) / (8.0 * e * i)
}

pub fn shear_stress_rectangular(v: f64, q: f64, i: f64, b: f64) -> f64 {
    v * q / (i * b)
}

pub fn section_modulus(i: f64, c: f64) -> f64 {
    i / c
}

pub fn radius_of_gyration(i: f64, area: f64) -> f64 {
    (i / area).sqrt()
}
