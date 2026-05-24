pub fn cubic_volume(a: f64) -> f64 {
    a.powi(3)
}

pub fn tetragonal_volume(a: f64, c: f64) -> f64 {
    a * a * c
}

pub fn orthorhombic_volume(a: f64, b: f64, c: f64) -> f64 {
    a * b * c
}

pub fn hexagonal_volume(a: f64, c: f64) -> f64 {
    3.0_f64.sqrt() / 2.0 * a * a * c
}

pub fn packing_fraction_simple_cubic() -> f64 {
    std::f64::consts::PI / 6.0
}

pub fn packing_fraction_bcc() -> f64 {
    std::f64::consts::PI * 3.0_f64.sqrt() / 8.0
}

pub fn packing_fraction_fcc() -> f64 {
    std::f64::consts::PI * 2.0_f64.sqrt() / 6.0
}

pub fn density_from_unit_cell(z: u32, molar_mass: f64, volume: f64, avogadro: f64) -> f64 {
    z as f64 * molar_mass / (avogadro * volume).max(1e-30)
}

pub fn miller_to_direction_cosines(h: i32, k: i32, l: i32) -> (f64, f64, f64) {
    let norm = ((h * h + k * k + l * l) as f64).sqrt().max(1e-30);
    (h as f64 / norm, k as f64 / norm, l as f64 / norm)
}

pub fn reciprocal_lattice_vector(a: f64) -> f64 {
    2.0 * std::f64::consts::PI / a.max(1e-30)
}
