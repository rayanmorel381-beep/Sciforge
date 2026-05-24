use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct HullKit {
    pub hull: StressedSkin,
    pub deck: StressedSkin,
}

pub fn patrol() -> HullKit {
    HullKit {
        hull: StressedSkin::aluminum(8.0, 95_000.0, 1_400.0),
        deck: StressedSkin::aluminum(5.0, 38_000.0, 420.0),
    }
}

pub fn workboat() -> HullKit {
    HullKit {
        hull: StressedSkin::aluminum(12.0, 220_000.0, 4_200.0),
        deck: StressedSkin::aluminum(8.0, 85_000.0, 1_100.0),
    }
}

pub fn all() -> Vec<HullKit> {
    vec![patrol(), workboat()]
}
