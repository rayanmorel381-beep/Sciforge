use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(2.0, 45_000.0, 42_000.0),
        wing_box: StressedSkin::aluminum(3.5, 120_000.0, 18_000.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
