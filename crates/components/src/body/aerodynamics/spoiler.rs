#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpoilerMaterial {
    Steel,
    Aluminum,
    CarbonFibre,
    ABS,
}

#[derive(Debug, Clone)]
pub struct Spoiler {
    pub material: SpoilerMaterial,
    pub height_mm: f64,
    pub width_mm: f64,
    pub adjustable: bool,
    pub active: bool,
}

impl Spoiler {
    pub fn fixed(material: SpoilerMaterial, height_mm: f64, width_mm: f64) -> Self {
        Self { material, height_mm, width_mm, adjustable: false, active: false }
    }

    pub fn adjustable(material: SpoilerMaterial, height_mm: f64, width_mm: f64) -> Self {
        Self { material, height_mm, width_mm, adjustable: true, active: false }
    }

    pub fn active(material: SpoilerMaterial, height_mm: f64, width_mm: f64) -> Self {
        Self { material, height_mm, width_mm, adjustable: true, active: true }
    }
}
