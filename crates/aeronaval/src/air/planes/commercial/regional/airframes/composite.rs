use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::carbon(1.2, 34_000.0, 13_500.0),
        wing_box: StressedSkin::carbon(2.2, 92_000.0, 7_200.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
