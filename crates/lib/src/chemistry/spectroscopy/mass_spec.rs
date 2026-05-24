pub fn mass_to_charge(mass: f64, charge: u32) -> f64 {
    mass / charge.max(1) as f64
}

pub fn exact_mass_difference(theoretical: f64, experimental: f64) -> f64 {
    (experimental - theoretical) / theoretical.max(1e-30) * 1e6
}

pub fn nitrogen_rule(nominal_mass: u32) -> bool {
    !nominal_mass.is_multiple_of(2)
}

pub fn rings_plus_double_bonds(c: u32, h: u32, n: u32, halogens: u32) -> f64 {
    (2.0 * c as f64 + 2.0 + n as f64 - h as f64 - halogens as f64) / 2.0
}

pub fn isotope_pattern_monoisotopic(abundances: &[f64]) -> f64 {
    abundances.iter().product()
}

pub fn resolving_power(m: f64, delta_m: f64) -> f64 {
    m / delta_m.max(1e-30)
}
