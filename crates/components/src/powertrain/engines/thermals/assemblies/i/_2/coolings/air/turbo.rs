use crate::cooling::intercooler::{
    Intercooler, IntercoolerType,
};

pub fn air_to_air() -> Intercooler {
    Intercooler::air_to_air(2.0)
}

pub fn air_to_air_large() -> Intercooler {
    Intercooler::air_to_air(3.0)
}

pub fn air_to_water() -> Intercooler {
    Intercooler::air_to_water(1.5)
}

pub fn water_spray() -> Intercooler {
    Intercooler::with_spray(IntercoolerType::AirToAir, 2.0)
}

pub fn twin_intercooler() -> Intercooler {
    Intercooler::air_to_air(4.0)
}
