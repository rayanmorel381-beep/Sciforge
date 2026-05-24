#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaliperType {
    Sliding,
    Fixed,
    Monoblock,
}

#[derive(Debug, Clone)]
pub struct Caliper {
    pub caliper_type: CaliperType,
    pub piston_count: u8,
    pub piston_diameter_mm: f64,
    pub aluminum_body: bool,
}

impl Caliper {
    pub fn sliding(piston_diameter_mm: f64) -> Self {
        Self { caliper_type: CaliperType::Sliding, piston_count: 1, piston_diameter_mm, aluminum_body: false }
    }

    pub fn fixed(piston_count: u8, piston_diameter_mm: f64) -> Self {
        Self { caliper_type: CaliperType::Fixed, piston_count, piston_diameter_mm, aluminum_body: true }
    }

    pub fn monoblock(piston_count: u8, piston_diameter_mm: f64) -> Self {
        Self { caliper_type: CaliperType::Monoblock, piston_count, piston_diameter_mm, aluminum_body: true }
    }
}
