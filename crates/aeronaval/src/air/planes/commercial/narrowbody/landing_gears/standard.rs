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
        main_rim: Rim::Alloy(AlloyRim::forged(26, 10.0, "26x10.0", 310.0, 18.5)),
        nose_rim: Rim::Alloy(AlloyRim::forged(22, 6.75, "22x6.75", 240.0, 9.0)),
        main_rotor: Rotor::carbon_ceramic(400.0, 40.0),
        main_caliper: Caliper::monoblock(8, 52.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![standard()]
}
