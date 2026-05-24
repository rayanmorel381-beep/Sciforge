use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(3.1)
}

pub fn air_to_water_chargecooler() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToWater, 3.9)
}

pub fn air_to_air_top_mount() -> Intercooler {
    Intercooler::air_to_air(3.9)
}
