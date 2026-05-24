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
        main_rim: Rim::Alloy(AlloyRim::forged(18, 7.0, "18x7.0", 175.0, 8.5)),
        nose_rim: Rim::Alloy(AlloyRim::forged(16, 5.5, "16x5.5", 155.0, 5.5)),
        main_rotor: Rotor::carbon_ceramic(300.0, 26.0),
        main_caliper: Caliper::monoblock(6, 40.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![standard()]
}
