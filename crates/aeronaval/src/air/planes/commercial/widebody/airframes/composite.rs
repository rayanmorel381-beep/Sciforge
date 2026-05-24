use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::carbon(2.0, 95_000.0, 68_000.0),
        wing_box: StressedSkin::carbon(4.2, 310_000.0, 38_000.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
