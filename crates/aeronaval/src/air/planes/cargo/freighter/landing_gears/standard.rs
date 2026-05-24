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
        main_rim: Rim::Alloy(AlloyRim::forged(30, 11.0, "30x11.0", 370.0, 27.0)),
        nose_rim: Rim::Alloy(AlloyRim::forged(26, 8.0, "26x8.0", 295.0, 14.0)),
        main_rotor: Rotor::carbon_ceramic(490.0, 52.0),
        main_caliper: Caliper::monoblock(10, 60.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![standard()]
}
