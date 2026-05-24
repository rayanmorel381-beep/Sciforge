pub mod flat;
pub mod radial;

#[derive(Debug, Clone)]
pub struct PistonEngine {
    pub displacement_cc: u32,
    pub bore_mm: f64,
    pub stroke_mm: f64,
    pub power_kw: f64,
    pub rpm_max: f64,
    pub compression_ratio: f64,
    pub cylinders: u8,
    pub turbocharged: bool,
}

pub fn all() -> Vec<PistonEngine> {
    let mut v = Vec::new();
    v.extend(flat::all());
    v.extend(radial::all());
    v
}
