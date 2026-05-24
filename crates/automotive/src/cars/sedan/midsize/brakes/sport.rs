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
        Rotor::drilled(345.0, 34.0),
        Caliper::fixed(4, 44.0),
        BrakePad::ceramic(),
    )
}

pub fn rear_sport() -> DiscKit {
    DiscKit::new(
        Rotor::drilled(318.0, 28.0),
        Caliper::fixed(2, 36.0),
        BrakePad::ceramic(),
    )
}
