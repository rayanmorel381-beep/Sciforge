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
        Rotor::cast_iron(280.0, 25.0, true),
        Caliper::sliding(54.0),
        BrakePad::semi_metallic(),
    )
}

pub fn front_fleet() -> DiscKit {
    DiscKit::new(
        Rotor::drilled(300.0, 28.0),
        Caliper::fixed(2, 38.0),
        BrakePad::semi_metallic(),
    )
}

pub fn rear_entry() -> DiscKit {
    DiscKit::new(
        Rotor::cast_iron(259.0, 10.0, false),
        Caliper::sliding(38.0),
        BrakePad::organic(),
    )
}

pub fn rear_fleet() -> DiscKit {
    DiscKit::new(
        Rotor::cast_iron(272.0, 22.0, true),
        Caliper::sliding(40.0),
        BrakePad::semi_metallic(),
    )
}
