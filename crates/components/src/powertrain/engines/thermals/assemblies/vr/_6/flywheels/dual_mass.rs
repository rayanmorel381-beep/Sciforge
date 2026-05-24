use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

pub fn standard() -> Flywheel { Flywheel::dual_mass(14.0, 250.0) }
