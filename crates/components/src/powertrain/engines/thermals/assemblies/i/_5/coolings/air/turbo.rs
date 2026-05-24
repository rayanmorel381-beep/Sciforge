use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_air() -> Intercooler {
    Intercooler::air_to_air(2.8)
}

pub fn air_to_air_large() -> Intercooler {
    Intercooler::air_to_air(4.2)
}

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(2.1)
}

pub fn water_spray() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToAir, 2.8)
}

pub fn twin_intercooler() -> Intercooler {
    Intercooler::air_to_air(5.6)
}
