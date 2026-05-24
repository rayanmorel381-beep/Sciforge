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

pub fn heavy() -> LandingGearKit {
    LandingGearKit {
        main_rim: Rim::Alloy(AlloyRim::forged(34, 13.0, "34x13.0", 440.0, 38.0)),
        nose_rim: Rim::Alloy(AlloyRim::forged(28, 9.0, "28x9.0", 335.0, 17.0)),
        main_rotor: Rotor::carbon_ceramic(540.0, 58.0),
        main_caliper: Caliper::fixed(12, 68.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![heavy()]
}
