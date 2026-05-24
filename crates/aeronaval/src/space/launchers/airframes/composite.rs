use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub core_stage: StressedSkin,
    pub fairing: StressedSkin,
}

pub fn light() -> AirframeKit {
    AirframeKit {
        core_stage: StressedSkin::carbon(3.5, 120_000.0, 2_400.0),
        fairing: StressedSkin::carbon(2.0, 45_000.0, 480.0),
    }
}

pub fn heavy() -> AirframeKit {
    AirframeKit {
        core_stage: StressedSkin::carbon(6.0, 380_000.0, 12_800.0),
        fairing: StressedSkin::carbon(4.0, 140_000.0, 2_200.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![light(), heavy()]
}
