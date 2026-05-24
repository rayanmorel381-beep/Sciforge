#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AirbagPosition {
    Frontal,
    Side,
    Curtain,
    Knee,
    SteeringWheel,
    Rear,
    Pedestrian,
}

#[derive(Debug, Clone)]
pub struct Airbag {
    pub position: AirbagPosition,
    pub dual_stage: bool,
    pub volume_l: f64,
}

impl Airbag {
    pub fn frontal(dual_stage: bool) -> Self {
        Self { position: AirbagPosition::Frontal, dual_stage, volume_l: 65.0 }
    }

    pub fn side(volume_l: f64) -> Self {
        Self { position: AirbagPosition::Side, dual_stage: false, volume_l }
    }

    pub fn curtain(volume_l: f64) -> Self {
        Self { position: AirbagPosition::Curtain, dual_stage: false, volume_l }
    }

    pub fn knee() -> Self {
        Self { position: AirbagPosition::Knee, dual_stage: false, volume_l: 18.0 }
    }
}
