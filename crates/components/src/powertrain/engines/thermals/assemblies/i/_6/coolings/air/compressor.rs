use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(2.3)
}

pub fn air_to_water_chargecooler() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToWater, 2.9)
}

pub fn air_to_air_top_mount() -> Intercooler {
    Intercooler::air_to_air(2.9)
}
