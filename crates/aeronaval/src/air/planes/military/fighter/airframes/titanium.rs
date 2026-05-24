use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(3.0, 62_000.0, 9_200.0),
        wing_box: StressedSkin::aluminum(4.5, 180_000.0, 4_800.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
