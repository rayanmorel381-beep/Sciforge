use super::hood::PanelMaterial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrunkStyle {
    Traditional,
    Hatchback,
    Liftgate,
    Tailgate,
}

#[derive(Debug, Clone)]
pub struct Trunk {
    pub style: TrunkStyle,
    pub material: PanelMaterial,
    pub power_release: bool,
    pub weight_kg: f64,
}

impl Trunk {
    pub fn traditional(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { style: TrunkStyle::Traditional, material, power_release: false, weight_kg }
    }

    pub fn hatchback(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { style: TrunkStyle::Hatchback, material, power_release: false, weight_kg }
    }

    pub fn power_liftgate(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { style: TrunkStyle::Liftgate, material, power_release: true, weight_kg }
    }

    pub fn tailgate(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { style: TrunkStyle::Tailgate, material, power_release: false, weight_kg }
    }
}
