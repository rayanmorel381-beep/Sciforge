use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub fuselage: StressedSkin,
    pub wing_box: StressedSkin,
}

pub fn standard() -> AirframeKit {
    AirframeKit {
        fuselage: StressedSkin::carbon(1.8, 75_000.0, 7_800.0),
        wing_box: StressedSkin::carbon(3.2, 220_000.0, 3_600.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![standard()]
}
