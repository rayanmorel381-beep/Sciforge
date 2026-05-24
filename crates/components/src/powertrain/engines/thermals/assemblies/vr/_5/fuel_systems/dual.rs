use crate::powertrain::engines::thermals::parts::fuel_systems::FuelSystem;
use super::super::CYLINDERS;

pub fn standard() -> FuelSystem { FuelSystem::dual_injection(CYLINDERS, 190.0, 220.0) }
