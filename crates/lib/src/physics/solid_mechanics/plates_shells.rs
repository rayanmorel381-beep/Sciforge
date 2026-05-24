pub fn flexural_rigidity(e: f64, h: f64, nu: f64) -> f64 {
    e * h.powi(3) / (12.0 * (1.0 - nu * nu))
}

pub fn kirchhoff_max_deflection_simply_supported(
    q: f64,
    a: f64,
    b: f64,
    e: f64,
    h: f64,
    nu: f64,
) -> f64 {
    let d = flexural_rigidity(e, h, nu);
    let alpha = if (a - b).abs() < 1e-12 { 0.00406 } else { 0.00581 };
    alpha * q * a.powi(4) / d
}

pub fn circular_plate_clamped_center_deflection(q: f64, r: f64, e: f64, h: f64, nu: f64) -> f64 {
    let d = flexural_rigidity(e, h, nu);
    q * r.powi(4) / (64.0 * d)
}

pub fn circular_plate_simply_supported_deflection(q: f64, r: f64, e: f64, h: f64, nu: f64) -> f64 {
    let d = flexural_rigidity(e, h, nu);
    q * r.powi(4) * (5.0 + nu) / (64.0 * d * (1.0 + nu))
}

pub fn membrane_stress_thin_sphere(p: f64, r: f64, t: f64) -> f64 {
    p * r / (2.0 * t)
}

pub fn membrane_stress_thin_cylinder_hoop(p: f64, r: f64, t: f64) -> f64 {
    p * r / t
}

pub fn membrane_stress_thin_cylinder_axial(p: f64, r: f64, t: f64) -> f64 {
    p * r / (2.0 * t)
}

pub fn mindlin_shear_correction(thickness: f64, length: f64, kappa: f64) -> f64 {
    let r = thickness / length;
    1.0 / (1.0 + kappa * r * r)
}
