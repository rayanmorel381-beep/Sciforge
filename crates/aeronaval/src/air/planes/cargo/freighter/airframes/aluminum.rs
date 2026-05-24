use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(3.5, 88_000.0, 105_000.0),
        wing_box: StressedSkin::aluminum(6.0, 340_000.0, 68_000.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
