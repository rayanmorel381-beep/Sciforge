use crate::components::fuel::tank::{FuelTank, FuelType, TankMaterial};

#[derive(Debug, Clone)]
pub struct PropellantKit {
    pub grain_tank: FuelTank,
}

pub fn sounding() -> PropellantKit {
    PropellantKit {
        grain_tank: FuelTank {
            fuel_type: FuelType::Kerosene,
            material: TankMaterial::CarbonFibre,
            capacity_l: 120.0,
            saddle: false,
            baffled: false,
            crash_resistant: false,
            fuel_level_sensor: false,
            in_tank_pump: false,
        },
    }
}

pub fn tactical() -> PropellantKit {
    PropellantKit {
        grain_tank: FuelTank {
            fuel_type: FuelType::Kerosene,
            material: TankMaterial::CarbonFibre,
            capacity_l: 1_800.0,
            saddle: false,
            baffled: false,
            crash_resistant: false,
            fuel_level_sensor: false,
            in_tank_pump: false,
        },
    }
}

pub fn all() -> Vec<PropellantKit> {
    vec![sounding(), tactical()]
}
