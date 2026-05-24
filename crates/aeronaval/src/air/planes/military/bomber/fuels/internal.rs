use crate::components::{
    fuel::tank::{FuelTank, FuelType, TankMaterial},
    powertrain::engines::turbines::assemblies::fuel::AvFuel,
};

#[derive(Debug, Clone)]
pub struct FuelKit {
    pub internal_tanks: Vec<FuelTank>,
    pub fuel: AvFuel,
}

pub fn internal() -> FuelKit {
    FuelKit {
        internal_tanks: vec![
            FuelTank {
                fuel_type: FuelType::Kerosene,
                material: TankMaterial::Aluminum,
                capacity_l: 75_000.0,
                saddle: false,
                baffled: true,
                crash_resistant: false,
                fuel_level_sensor: true,
                in_tank_pump: true,
            },
        ],
        fuel: AvFuel::Jp8,
    }
}

pub fn all() -> Vec<FuelKit> {
    vec![internal()]
}
