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
        main_rim: Rim::Alloy(AlloyRim::forged(19, 7.0, "19x7.0", 200.0, 10.0)),
        nose_rim: Rim::Alloy(AlloyRim::forged(17, 5.0, "17x5.0", 175.0, 6.0)),
        main_rotor: Rotor::carbon_ceramic(340.0, 32.0),
        main_caliper: Caliper::monoblock(6, 46.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![standard()]
}
