use crate::components::safety::{Airbag, CrumpleZone, Seatbelt};

#[derive(Debug, Clone)]
pub struct SafetyKit {
    pub airbags: Vec<Airbag>,
    pub seatbelts: Vec<Seatbelt>,
    pub crumple_zones: Vec<CrumpleZone>,
}

pub fn sport() -> SafetyKit {
    SafetyKit {
        airbags: vec![
            Airbag::frontal(true),
            Airbag::side(12.0),
            Airbag::curtain(25.0),
            Airbag::knee(),
        ],
        seatbelts: vec![
            Seatbelt::three_point(),
            Seatbelt::three_point(),
            Seatbelt::three_point(),
            Seatbelt::three_point(),
            Seatbelt::three_point(),
        ],
        crumple_zones: vec![
            CrumpleZone::front(220.0),
            CrumpleZone::rear(150.0),
            CrumpleZone::side(110.0),
        ],
    }
}

pub fn all() -> Vec<SafetyKit> {
    vec![sport()]
}
