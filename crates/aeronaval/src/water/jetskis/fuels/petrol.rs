use crate::components::fuel::tank::{FuelTank, FuelType, TankMaterial};

#[derive(Debug, Clone)]
pub struct FuelKit {
    pub tank: FuelTank,
}

pub fn classic() -> FuelKit {
    FuelKit {
        tank: FuelTank {
            fuel_type: FuelType::Gasoline,
            material: TankMaterial::Hdpe,
            capacity_l: 42.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        },
    }
}

pub fn supercharged() -> FuelKit {
    FuelKit {
        tank: FuelTank {
            fuel_type: FuelType::Gasoline,
            material: TankMaterial::Aluminum,
            capacity_l: 70.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        },
    }
}

pub fn all() -> Vec<FuelKit> {
    vec![classic(), supercharged()]
}
