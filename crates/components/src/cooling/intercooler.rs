#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntercoolerType {
    AirToAir,
    AirToWater,
}

#[derive(Debug, Clone)]
pub struct Intercooler {
    pub cooler_type: IntercoolerType,
    pub volume_l: f64,
    pub pressure_drop_kpa: f64,
    pub spray: bool,
}

impl Intercooler {
    pub fn air_to_air(volume_l: f64) -> Self {
        Self { cooler_type: IntercoolerType::AirToAir, volume_l, pressure_drop_kpa: 8.0, spray: false }
    }

    pub fn air_to_water(volume_l: f64) -> Self {
        Self { cooler_type: IntercoolerType::AirToWater, volume_l, pressure_drop_kpa: 5.0, spray: false }
    }

    pub fn with_spray(cooler_type: IntercoolerType, volume_l: f64) -> Self {
        Self { cooler_type, volume_l, pressure_drop_kpa: 5.0, spray: true }
    }
}
