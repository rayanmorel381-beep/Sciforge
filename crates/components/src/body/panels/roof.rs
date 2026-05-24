use super::hood::PanelMaterial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoofType {
    Fixed,
    Convertible,
    Targa,
    Panoramic,
    Retractable,
}

#[derive(Debug, Clone)]
pub struct Roof {
    pub roof_type: RoofType,
    pub material: PanelMaterial,
    pub weight_kg: f64,
}

impl Roof {
    pub fn fixed(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { roof_type: RoofType::Fixed, material, weight_kg }
    }

    pub fn convertible(weight_kg: f64) -> Self {
        Self { roof_type: RoofType::Convertible, material: PanelMaterial::Steel, weight_kg }
    }

    pub fn targa(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { roof_type: RoofType::Targa, material, weight_kg }
    }

    pub fn panoramic(weight_kg: f64) -> Self {
        Self { roof_type: RoofType::Panoramic, material: PanelMaterial::Steel, weight_kg }
    }
}
