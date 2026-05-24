use super::hood::PanelMaterial;

#[derive(Debug, Clone)]
pub struct Fender {
    pub material: PanelMaterial,
    pub widebody: bool,
    pub flared_mm: f64,
    pub weight_kg: f64,
}

impl Fender {
    pub fn standard(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { material, widebody: false, flared_mm: 0.0, weight_kg }
    }

    pub fn widebody(material: PanelMaterial, flared_mm: f64, weight_kg: f64) -> Self {
        Self { material, widebody: true, flared_mm, weight_kg }
    }
}
