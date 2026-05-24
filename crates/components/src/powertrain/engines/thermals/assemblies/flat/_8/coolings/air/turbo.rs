use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_air() -> Intercooler {
    Intercooler::air_to_air(3.8)
}

pub fn air_to_air_top_mount() -> Intercooler {
    Intercooler::air_to_air(4.6)
}

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(3.0)
}

pub fn water_spray() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToAir, 3.8)
}
