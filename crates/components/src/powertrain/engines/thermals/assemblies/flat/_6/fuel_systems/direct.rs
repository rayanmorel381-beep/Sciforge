use crate::powertrain::engines::thermals::parts::fuel_systems::FuelSystem;

use super::super::CYLINDERS;

pub fn standard() -> FuelSystem {
    FuelSystem::direct_injection(CYLINDERS, 320.0)
}

pub fn high_flow() -> FuelSystem {
    FuelSystem::direct_injection(CYLINDERS, 480.0)
}
