use super::hood::PanelMaterial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorType {
    Conventional,
    Suicide,
    Scissor,
    Butterfly,
    Gullwing,
    Sliding,
}

#[derive(Debug, Clone)]
pub struct Door {
    pub door_type: DoorType,
    pub material: PanelMaterial,
    pub frameless: bool,
    pub weight_kg: f64,
}

impl Door {
    pub fn conventional(material: PanelMaterial, frameless: bool, weight_kg: f64) -> Self {
        Self { door_type: DoorType::Conventional, material, frameless, weight_kg }
    }

    pub fn scissor(weight_kg: f64) -> Self {
        Self { door_type: DoorType::Scissor, material: PanelMaterial::Aluminum, frameless: true, weight_kg }
    }

    pub fn gullwing(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { door_type: DoorType::Gullwing, material, frameless: false, weight_kg }
    }

    pub fn sliding(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { door_type: DoorType::Sliding, material, frameless: false, weight_kg }
    }
}
