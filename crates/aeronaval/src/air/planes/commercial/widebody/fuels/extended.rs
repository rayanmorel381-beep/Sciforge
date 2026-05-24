use crate::components::{
    fuel::tank::{FuelTank, FuelType, TankMaterial},
    powertrain::engines::turbines::assemblies::fuel::AvFuel,
};

#[derive(Debug, Clone)]
pub struct FuelKit {
    pub wing_tanks: Vec<FuelTank>,
    pub center_tank: Option<FuelTank>,
    pub trim_tank: Option<FuelTank>,
    pub fuel: AvFuel,
}

pub fn extended() -> FuelKit {
    FuelKit {
        wing_tanks: vec![
            FuelTank {
                fuel_type: FuelType::Kerosene,
                material: TankMaterial::Aluminum,
                capacity_l: 62_000.0,
                saddle: false,
                baffled: true,
                crash_resistant: false,
                fuel_level_sensor: true,
                in_tank_pump: true,
            },
            FuelTank {
                fuel_type: FuelType::Kerosene,
                material: TankMaterial::Aluminum,
                capacity_l: 62_000.0,
                saddle: false,
                baffled: true,
                crash_resistant: false,
                fuel_level_sensor: true,
                in_tank_pump: true,
            },
        ],
        center_tank: Some(FuelTank {
            fuel_type: FuelType::Kerosene,
            material: TankMaterial::Aluminum,
            capacity_l: 110_000.0,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        }),
        trim_tank: Some(FuelTank {
            fuel_type: FuelType::Kerosene,
            material: TankMaterial::Aluminum,
            capacity_l: 6_000.0,
            saddle: false,
            baffled: false,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: false,
        }),
        fuel: AvFuel::JetA,
    }
}

pub fn all() -> Vec<FuelKit> {
    vec![extended()]
}
