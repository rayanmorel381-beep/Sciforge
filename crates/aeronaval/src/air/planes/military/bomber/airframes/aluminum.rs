use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::aluminum(4.0, 110_000.0, 120_000.0),
        wing_box: StressedSkin::aluminum(6.5, 380_000.0, 65_000.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
