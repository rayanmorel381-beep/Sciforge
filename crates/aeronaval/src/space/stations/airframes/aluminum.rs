use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub pressure_hull: StressedSkin,
    pub truss: StressedSkin,
}

pub fn habitat_module() -> AirframeKit {
    AirframeKit {
        pressure_hull: StressedSkin::aluminum(4.8, 280_000.0, 4_500.0),
        truss: StressedSkin::aluminum(3.0, 95_000.0, 1_800.0),
    }
}

pub fn science_module() -> AirframeKit {
    AirframeKit {
        pressure_hull: StressedSkin::aluminum(4.0, 210_000.0, 3_200.0),
        truss: StressedSkin::aluminum(2.5, 72_000.0, 1_200.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![habitat_module(), science_module()]
}
