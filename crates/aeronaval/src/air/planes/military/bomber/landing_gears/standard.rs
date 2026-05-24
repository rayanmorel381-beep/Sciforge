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
        main_rim: Rim::Alloy(AlloyRim::forged(26, 9.0, "26x9.0", 290.0, 21.0)),
        nose_rim: Rim::Alloy(AlloyRim::forged(22, 7.0, "22x7.0", 240.0, 12.0)),
        main_rotor: Rotor::carbon_ceramic(440.0, 44.0),
        main_caliper: Caliper::monoblock(10, 56.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![standard()]
}
