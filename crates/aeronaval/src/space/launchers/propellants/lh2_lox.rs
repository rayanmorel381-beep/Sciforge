use crate::components::fuel::tank::{FuelTank, FuelType, TankMaterial};

#[derive(Debug, Clone)]
pub struct PropellantKit {
    pub fuel_tank: FuelTank,
    pub oxidizer_tank: FuelTank,
}

pub fn heavy() -> PropellantKit {
    PropellantKit {
        fuel_tank: FuelTank {
            fuel_type: FuelType::LiquidHydrogen,
            material: TankMaterial::Aluminum,
            capacity_l: 1_460_000.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        },
        oxidizer_tank: FuelTank {
            fuel_type: FuelType::LiquidOxygen,
            material: TankMaterial::Aluminum,
            capacity_l: 870_000.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        },
    }
}

pub fn all() -> Vec<PropellantKit> {
    vec![heavy()]
}
