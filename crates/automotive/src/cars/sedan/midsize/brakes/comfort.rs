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

pub fn front_entry() -> DiscKit {
    DiscKit::new(
        Rotor::cast_iron(292.0, 26.0, true),
        Caliper::sliding(54.0),
        BrakePad::semi_metallic(),
    )
}

pub fn front_comfort() -> DiscKit {
    DiscKit::new(
        Rotor::drilled(310.0, 29.0),
        Caliper::fixed(2, 40.0),
        BrakePad::ceramic(),
    )
}

pub fn front_premium() -> DiscKit {
    DiscKit::new(
        Rotor::drilled(330.0, 32.0),
        Caliper::fixed(4, 42.0),
        BrakePad::ceramic(),
    )
}

pub fn rear_entry() -> DiscKit {
    DiscKit::new(
        Rotor::cast_iron(268.0, 11.0, false),
        Caliper::sliding(38.0),
        BrakePad::organic(),
    )
}

pub fn rear_comfort() -> DiscKit {
    DiscKit::new(
        Rotor::cast_iron(282.0, 23.0, true),
        Caliper::sliding(40.0),
        BrakePad::semi_metallic(),
    )
}

pub fn rear_premium() -> DiscKit {
    DiscKit::new(
        Rotor::drilled(300.0, 25.0),
        Caliper::fixed(2, 34.0),
        BrakePad::ceramic(),
    )
}
