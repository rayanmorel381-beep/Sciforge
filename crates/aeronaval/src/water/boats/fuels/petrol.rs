use crate::components::fuel::tank::{FuelTank, FuelType, TankMaterial};

#[derive(Debug, Clone)]
pub struct FuelKit {
    pub tank: FuelTank,
}

pub fn outboard_small() -> FuelKit {
    FuelKit {
        tank: FuelTank {
            fuel_type: FuelType::Gasoline,
            material: TankMaterial::Hdpe,
            capacity_l: 12.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: false,
        },
    }
}

pub fn outboard_mid() -> FuelKit {
    FuelKit {
        tank: FuelTank {
            fuel_type: FuelType::Gasoline,
            material: TankMaterial::Aluminum,
            capacity_l: 90.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: false,
        },
    }
}

pub fn all() -> Vec<FuelKit> {
    vec![outboard_small(), outboard_mid()]
}
