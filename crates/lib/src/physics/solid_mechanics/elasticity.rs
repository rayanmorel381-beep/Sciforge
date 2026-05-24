pub fn hooke_stress(e: f64, strain: f64) -> f64 {
    e * strain
}

pub fn hooke_strain(stress: f64, e: f64) -> f64 {
    stress / e
}

pub fn poisson_lateral_strain(axial_strain: f64, nu: f64) -> f64 {
    -nu * axial_strain
}

pub fn shear_modulus(e: f64, nu: f64) -> f64 {
    e / (2.0 * (1.0 + nu))
}

pub fn bulk_modulus(e: f64, nu: f64) -> f64 {
    e / (3.0 * (1.0 - 2.0 * nu))
}

pub fn lame_first(e: f64, nu: f64) -> f64 {
    e * nu / ((1.0 + nu) * (1.0 - 2.0 * nu))
}

pub fn plane_stress_strain(stress_x: f64, stress_y: f64, e: f64, nu: f64) -> (f64, f64) {
    let ex = (stress_x - nu * stress_y) / e;
    let ey = (stress_y - nu * stress_x) / e;
    (ex, ey)
}

pub fn strain_energy_density(stress: f64, strain: f64) -> f64 {
    0.5 * stress * strain
}

pub fn thermal_strain(alpha: f64, delta_t: f64) -> f64 {
    alpha * delta_t
}

pub fn thermal_stress(e: f64, alpha: f64, delta_t: f64) -> f64 {
    -e * alpha * delta_t
}

pub fn volumetric_strain(ex: f64, ey: f64, ez: f64) -> f64 {
    ex + ey + ez
}

pub fn hydrostatic_stress(sx: f64, sy: f64, sz: f64) -> f64 {
    (sx + sy + sz) / 3.0
}
