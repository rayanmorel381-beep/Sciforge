use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

pub fn standard() -> Flywheel { Flywheel::single_mass(9.5, 235.0) }
