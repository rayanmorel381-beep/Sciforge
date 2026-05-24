use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

const MASS_KG: f64 = 4.5;
const DIAMETER_MM: f64 = 220.0;

pub fn standard() -> Flywheel {
    Flywheel::lightweight(MASS_KG, DIAMETER_MM)
}
