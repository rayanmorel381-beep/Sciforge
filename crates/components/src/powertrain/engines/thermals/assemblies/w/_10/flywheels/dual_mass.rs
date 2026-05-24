use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

pub fn standard() -> Flywheel { Flywheel::dual_mass(20.0, 280.0) }
