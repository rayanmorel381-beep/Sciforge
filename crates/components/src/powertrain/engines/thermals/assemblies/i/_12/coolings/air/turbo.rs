use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_air() -> Intercooler {
    Intercooler::air_to_air(7.0)
}

pub fn air_to_air_large() -> Intercooler {
    Intercooler::air_to_air(9.8)
}

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(4.9)
}

pub fn water_spray() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToAir, 7.0)
}

pub fn twin_intercooler() -> Intercooler {
    Intercooler::air_to_air(14.0)
}
