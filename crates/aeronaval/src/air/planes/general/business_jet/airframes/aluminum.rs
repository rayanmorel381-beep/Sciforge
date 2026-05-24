use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(1.4, 18_000.0, 5_800.0),
        wing_box: StressedSkin::aluminum(2.2, 58_000.0, 4_200.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
