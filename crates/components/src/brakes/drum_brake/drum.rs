#[derive(Debug, Clone)]
pub struct Drum {
    pub diameter_mm: f64,
    pub width_mm: f64,
    pub cast_iron: bool,
    pub finned: bool,
}

impl Drum {
    pub fn standard(diameter_mm: f64, width_mm: f64) -> Self {
        Self { diameter_mm, width_mm, cast_iron: true, finned: false }
    }

    pub fn finned(diameter_mm: f64, width_mm: f64) -> Self {
        Self { diameter_mm, width_mm, cast_iron: true, finned: true }
    }
}
