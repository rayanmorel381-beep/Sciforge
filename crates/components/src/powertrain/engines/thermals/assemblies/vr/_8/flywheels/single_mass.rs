use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

pub fn standard() -> Flywheel { Flywheel::single_mass(13.0, 260.0) }
