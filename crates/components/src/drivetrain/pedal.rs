#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PedalType {
    Platform,
    Clipless,
    ToeClip,
    HybridClip,
}

#[derive(Debug, Clone)]
pub struct Pedal {
    pub pedal_type: PedalType,
    pub body_material: PedalMaterial,
    pub platform_area_cm2: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PedalMaterial {
    Resin,
    Aluminum,
    CarbonFibre,
    Steel,
}

impl Pedal {
    pub fn platform(body_material: PedalMaterial, platform_area_cm2: f64) -> Self {
        Self { pedal_type: PedalType::Platform, body_material, platform_area_cm2 }
    }

    pub fn clipless(body_material: PedalMaterial) -> Self {
        Self { pedal_type: PedalType::Clipless, body_material, platform_area_cm2: 30.0 }
    }
}
