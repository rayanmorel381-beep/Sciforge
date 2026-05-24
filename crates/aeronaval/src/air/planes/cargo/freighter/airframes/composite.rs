use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::carbon(2.2, 108_000.0, 76_000.0),
        wing_box: StressedSkin::carbon(4.5, 430_000.0, 49_000.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
