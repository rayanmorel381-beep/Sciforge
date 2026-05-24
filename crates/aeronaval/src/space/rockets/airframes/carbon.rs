use crate::components::frames::monocoque::StressedSkin;

#[derive(Debug, Clone)]
pub struct AirframeKit {
    pub body: StressedSkin,
    pub nosecone: StressedSkin,
}

pub fn sounding() -> AirframeKit {
    AirframeKit {
        body: StressedSkin::carbon(2.0, 18_000.0, 28.0),
        nosecone: StressedSkin::carbon(1.5, 8_000.0, 6.0),
    }
}

pub fn tactical() -> AirframeKit {
    AirframeKit {
        body: StressedSkin::carbon(4.0, 55_000.0, 180.0),
        nosecone: StressedSkin::carbon(3.0, 22_000.0, 35.0),
    }
}

pub fn all() -> Vec<AirframeKit> {
    vec![sounding(), tactical()]
}
