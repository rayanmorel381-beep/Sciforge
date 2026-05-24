use crate::components::brakes::{BrakePad, Caliper, Rotor, BrakeShoe, Drum};

#[derive(Debug, Clone)]
pub struct DiscKit {
    pub rotor: Rotor,
    pub caliper: Caliper,
    pub pad: BrakePad,
}

impl DiscKit {
    pub fn new(rotor: Rotor, caliper: Caliper, pad: BrakePad) -> Self {
        Self { rotor, caliper, pad }
    }
}

#[derive(Debug, Clone)]
pub struct DrumKit {
    pub drum: Drum,
    pub shoe: BrakeShoe,
}

pub fn front_entry() -> DiscKit {
    DiscKit::new(
        Rotor::cast_iron(280.0, 25.0, true),
        Caliper::sliding(54.0),
        BrakePad::semi_metallic(),
    )
}

pub fn rear_entry() -> DrumKit {
    DrumKit {
        drum: Drum::standard(200.0, 38.0),
        shoe: BrakeShoe::leading_trailing(38.0),
    }
}
