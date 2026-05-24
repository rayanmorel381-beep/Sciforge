use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(4.7)
}

pub fn air_to_water_chargecooler() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToWater, 5.9)
}

pub fn air_to_air_top_mount() -> Intercooler {
    Intercooler::air_to_air(5.9)
}
