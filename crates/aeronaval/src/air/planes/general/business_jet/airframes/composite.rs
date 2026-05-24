use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::carbon(1.0, 22_000.0, 4_200.0),
        wing_box: StressedSkin::carbon(1.8, 72_000.0, 3_100.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
