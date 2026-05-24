use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(1.6, 28_000.0, 18_000.0),
        wing_box: StressedSkin::aluminum(2.8, 75_000.0, 9_500.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
