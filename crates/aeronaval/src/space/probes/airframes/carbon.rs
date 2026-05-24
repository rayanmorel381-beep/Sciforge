use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub bus: StressedSkin,
    pub instrument_panel: StressedSkin,
}

pub fn cubesat() -> AirframeKit {
    AirframeKit {
        bus: StressedSkin::carbon(1.0, 2_500.0, 1.2),
        instrument_panel: StressedSkin::carbon(0.8, 900.0, 0.3),
    }
}

pub fn deep_space() -> AirframeKit {
    AirframeKit {
        bus: StressedSkin::carbon(2.5, 18_000.0, 120.0),
        instrument_panel: StressedSkin::carbon(1.8, 6_500.0, 28.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![cubesat(), deep_space()]
}
