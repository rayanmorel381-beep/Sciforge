use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_air() -> Intercooler {
    Intercooler::air_to_air(4.6)
}

pub fn air_to_air_large() -> Intercooler {
    Intercooler::air_to_air(6.6)
}

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(3.3)
}

pub fn water_spray() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToAir, 4.6)
}

pub fn twin_intercooler() -> Intercooler {
    Intercooler::air_to_air(9.2)
}
