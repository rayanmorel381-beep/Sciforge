use crate::components::{
    brakes::disc::{Caliper, Rotor},
    wheels::rims::{AlloyRim, Rim},
};

#[derive(Debug, Clone)]
pub struct LandingGearKit {
    pub main_rim: Rim,
    pub nose_rim: Rim,
    pub main_rotor: Rotor,
    pub main_caliper: Caliper,
}

pub fn standard() -> LandingGearKit {
    LandingGearKit {
        main_rim: Rim::Alloy(AlloyRim::forged(15, 6.0, "15x6.0", 147.0, 5.0)),
        nose_rim: Rim::Alloy(AlloyRim::forged(13, 4.5, "13x4.5", 127.0, 3.2)),
        main_rotor: Rotor::cast_iron(235.0, 18.0, true),
        main_caliper: Caliper::monoblock(4, 32.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![standard()]
}
