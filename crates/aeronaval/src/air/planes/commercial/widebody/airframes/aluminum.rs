use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(3.2, 78_000.0, 95_000.0),
        wing_box: StressedSkin::aluminum(5.5, 240_000.0, 52_000.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
