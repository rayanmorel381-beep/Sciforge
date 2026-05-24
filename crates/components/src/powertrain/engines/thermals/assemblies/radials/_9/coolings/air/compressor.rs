use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(1.6)
}

pub fn air_to_water_chargecooler() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToWater, 2.0)
}

pub fn air_to_air_top_mount() -> Intercooler {
    Intercooler::air_to_air(2.0)
}
