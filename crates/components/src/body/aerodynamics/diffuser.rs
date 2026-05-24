#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffuserMaterial {
    CarbonFibre,
    Aluminum,
    Composite,
}

#[derive(Debug, Clone)]
pub struct Diffuser {
    pub material: DiffuserMaterial,
    pub length_mm: f64,
    pub width_mm: f64,
    pub fin_count: u8,
    pub active: bool,
}

impl Diffuser {
    pub fn passive(material: DiffuserMaterial, length_mm: f64, width_mm: f64, fin_count: u8) -> Self {
        Self { material, length_mm, width_mm, fin_count, active: false }
    }

    pub fn active(material: DiffuserMaterial, length_mm: f64, width_mm: f64, fin_count: u8) -> Self {
        Self { material, length_mm, width_mm, fin_count, active: true }
    }
}
