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
        main_rim: Rim::Alloy(AlloyRim::forged(28, 10.0, "28x10.0", 345.0, 25.0)),
        nose_rim: Rim::Alloy(AlloyRim::forged(24, 7.5, "24x7.5", 280.0, 13.0)),
        main_rotor: Rotor::carbon_ceramic(460.0, 46.0),
        main_caliper: Caliper::monoblock(10, 58.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![standard()]
}
