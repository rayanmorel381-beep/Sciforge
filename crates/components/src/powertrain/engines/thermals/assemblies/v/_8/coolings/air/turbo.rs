use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_air() -> Intercooler {
    Intercooler::air_to_air(4.0)
}

pub fn air_to_air_top_mount() -> Intercooler {
    Intercooler::air_to_air(4.8)
}

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(3.2)
}

pub fn water_spray() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToAir, 4.0)
}
