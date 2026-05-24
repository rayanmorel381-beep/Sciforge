use crate::components::safety::{Airbag, CrumpleZone, Seatbelt};

#[derive(Debug, Clone)]
pub struct SafetyKit {
    pub airbags: Vec<Airbag>,
    pub seatbelts: Vec<Seatbelt>,
    pub crumple_zones: Vec<CrumpleZone>,
}

pub fn standard() -> SafetyKit {
    SafetyKit {
        airbags: vec![
            Airbag::frontal(false),
            Airbag::side(12.0),
            Airbag::curtain(25.0),
        ],
        seatbelts: vec![
            Seatbelt::three_point(),
            Seatbelt::three_point(),
            Seatbelt::three_point(),
            Seatbelt::three_point(),
            Seatbelt::three_point(),
        ],
        crumple_zones: vec![
            CrumpleZone::front(180.0),
            CrumpleZone::rear(120.0),
            CrumpleZone::side(80.0),
        ],
    }
}

pub fn all() -> Vec<SafetyKit> {
    vec![standard()]
}
