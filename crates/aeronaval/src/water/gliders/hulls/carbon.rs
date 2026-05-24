use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct HullKit {
    pub pressure_hull: StressedSkin,
    pub fairing: StressedSkin,
}

pub fn auv() -> HullKit {
    HullKit {
        pressure_hull: StressedSkin::carbon(5.0, 60_000.0, 24.0),
        fairing: StressedSkin::carbon(2.5, 18_000.0, 7.0),
    }
}

pub fn all() -> Vec<HullKit> {
    vec![auv()]
}
