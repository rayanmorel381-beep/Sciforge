use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::carbon(2.5, 135_000.0, 88_000.0),
        wing_box: StressedSkin::carbon(4.8, 480_000.0, 48_000.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
