use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_air() -> Intercooler {
    Intercooler::air_to_air(2.6)
}

pub fn air_to_air_top_mount() -> Intercooler {
    Intercooler::air_to_air(3.4)
}

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(2.0)
}

pub fn water_spray() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToAir, 2.6)
}
