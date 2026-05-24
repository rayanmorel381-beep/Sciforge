use crate::components::fuel::tank::{FuelTank, FuelType, TankMaterial};

#[derive(Debug, Clone)]
pub struct FuelKit {
    pub tank: FuelTank,
}

pub fn inboard_cruiser() -> FuelKit {
    FuelKit {
        tank: FuelTank {
            fuel_type: FuelType::Diesel,
            material: TankMaterial::Aluminum,
            capacity_l: 600.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: false,
        },
    }
}

pub fn stern_drive() -> FuelKit {
    FuelKit {
        tank: FuelTank {
            fuel_type: FuelType::Diesel,
            material: TankMaterial::Aluminum,
            capacity_l: 320.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: false,
        },
    }
}

pub fn all() -> Vec<FuelKit> {
    vec![inboard_cruiser(), stern_drive()]
}
