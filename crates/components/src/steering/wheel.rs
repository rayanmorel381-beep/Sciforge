#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelMaterial {
    Leather,
    AlcantaraLeather,
    CarbonLeather,
    Wood,
    Urethane,
}

#[derive(Debug, Clone)]
pub struct SteeringWheel {
    pub diameter_mm: f64,
    pub material: WheelMaterial,
    pub heated: bool,
    pub flat_bottom: bool,
    pub paddle_shifters: bool,
}

impl SteeringWheel {
    pub fn standard(diameter_mm: f64, material: WheelMaterial) -> Self {
        Self { diameter_mm, material, heated: false, flat_bottom: false, paddle_shifters: false }
    }

    pub fn sport(diameter_mm: f64, material: WheelMaterial) -> Self {
        Self { diameter_mm, material, heated: false, flat_bottom: true, paddle_shifters: true }
    }

    pub fn heated(diameter_mm: f64, material: WheelMaterial) -> Self {
        Self { diameter_mm, material, heated: true, flat_bottom: false, paddle_shifters: false }
    }
}
