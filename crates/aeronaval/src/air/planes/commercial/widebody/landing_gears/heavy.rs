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
        main_rim: Rim::Alloy(AlloyRim::forged(32, 12.0, "32x12.0", 420.0, 34.0)),
        nose_rim: Rim::Alloy(AlloyRim::forged(28, 9.0, "28x9.0", 340.0, 17.0)),
        main_rotor: Rotor::carbon_ceramic(520.0, 55.0),
        main_caliper: Caliper::fixed(12, 65.0),
    }
}

pub fn all() -> Vec<LandingGearKit> {
    vec![heavy()]
}
