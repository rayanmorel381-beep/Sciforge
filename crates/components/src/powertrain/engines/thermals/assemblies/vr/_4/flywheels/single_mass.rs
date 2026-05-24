use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

pub fn standard() -> Flywheel { Flywheel::single_mass(8.5, 220.0) }
