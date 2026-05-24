use crate::components::{
    fuel::tank::{FuelTank, FuelType, TankMaterial},
    powertrain::engines::turbines::assemblies::fuel::AvFuel,
};

#[derive(Debug, Clone)]
pub struct FuelKit {
    pub wing_tanks: Vec<FuelTank>,
    pub fuel: AvFuel,
}

pub fn standard() -> FuelKit {
    FuelKit {
        wing_tanks: vec![
            FuelTank {
                fuel_type: FuelType::Kerosene,
                material: TankMaterial::Aluminum,
                capacity_l: 120.0,
                saddle: false,
                baffled: false,
                crash_resistant: false,
                fuel_level_sensor: true,
                in_tank_pump: false,
            },
            FuelTank {
                fuel_type: FuelType::Kerosene,
                material: TankMaterial::Aluminum,
                capacity_l: 120.0,
                saddle: false,
                baffled: false,
                crash_resistant: false,
                fuel_level_sensor: true,
                in_tank_pump: false,
            },
        ],
        fuel: AvFuel::AvGas100Ll,
    }
}

pub fn all() -> Vec<FuelKit> {
    vec![standard()]
}
