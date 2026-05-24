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

pub fn extended_range() -> LandingGearKit {
    LandingGearKit {
        main_rim: Rim::Alloy(AlloyRim::forged(27, 10.5, "27x10.5", 320.0, 20.0)),
        nose_rim: Rim::Alloy(AlloyRim::forged(22, 7.0, "22x7.0", 250.0, 9.5)),
        main_rotor: Rotor::carbon_ceramic(420.0, 42.0),
        main_caliper: Caliper::monoblock(10, 54.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![extended_range()]
}
