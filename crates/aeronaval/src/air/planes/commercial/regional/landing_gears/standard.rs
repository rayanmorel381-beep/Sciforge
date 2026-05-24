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
        main_rim: Rim::Alloy(AlloyRim::forged(20, 8.0, "20x8.0", 235.0, 11.5)),
        nose_rim: Rim::Alloy(AlloyRim::forged(17, 5.5, "17x5.5", 190.0, 6.5)),
        main_rotor: Rotor::cast_iron(350.0, 30.0, true),
        main_caliper: Caliper::monoblock(6, 44.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![standard()]
}
