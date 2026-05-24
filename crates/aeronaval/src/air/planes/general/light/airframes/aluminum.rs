use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(0.8, 8_500.0, 520.0),
        wing_box: StressedSkin::aluminum(1.2, 22_000.0, 680.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
