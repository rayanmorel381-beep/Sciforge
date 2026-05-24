use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(3.8, 95_000.0, 85_000.0),
        wing_box: StressedSkin::aluminum(6.0, 310_000.0, 55_000.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
