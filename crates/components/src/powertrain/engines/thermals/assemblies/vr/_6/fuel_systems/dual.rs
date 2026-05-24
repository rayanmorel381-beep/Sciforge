use crate::powertrain::engines::thermals::parts::fuel_systems::FuelSystem;
use super::super::CYLINDERS;

pub fn standard() -> FuelSystem { FuelSystem::dual_injection(CYLINDERS, 320.0, 340.0) }
