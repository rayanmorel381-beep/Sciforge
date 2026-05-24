use crate::components::brakes::{BrakePad, Caliper, Rotor};

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

pub fn front_sport() -> DiscKit {
    DiscKit::new(
        Rotor::drilled(330.0, 32.0),
        Caliper::fixed(4, 42.0),
        BrakePad::ceramic(),
    )
}

pub fn rear_sport() -> DiscKit {
    DiscKit::new(
        Rotor::drilled(305.0, 26.0),
        Caliper::fixed(2, 34.0),
        BrakePad::ceramic(),
    )
}
