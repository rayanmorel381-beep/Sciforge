use crate::components::fuel::tank::{FuelTank, FuelType, TankMaterial};

#[derive(Debug, Clone)]
pub struct PropellantKit {
    pub fuel_tank: FuelTank,
    pub oxidizer_tank: FuelTank,
}

pub fn light() -> PropellantKit {
    PropellantKit {
        fuel_tank: FuelTank {
            fuel_type: FuelType::Kerosene,
            material: TankMaterial::Aluminum,
            capacity_l: 55_000.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        },
        oxidizer_tank: FuelTank {
            fuel_type: FuelType::LiquidOxygen,
            material: TankMaterial::Aluminum,
            capacity_l: 121_000.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        },
    }
}

pub fn medium() -> PropellantKit {
    PropellantKit {
        fuel_tank: FuelTank {
            fuel_type: FuelType::Kerosene,
            material: TankMaterial::Aluminum,
            capacity_l: 175_000.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        },
        oxidizer_tank: FuelTank {
            fuel_type: FuelType::LiquidOxygen,
            material: TankMaterial::Aluminum,
            capacity_l: 385_000.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        },
    }
}

pub fn all() -> Vec<PropellantKit> {
    vec![light(), medium()]
}
