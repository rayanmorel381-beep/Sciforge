use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

const MASS_KG: f64 = 28.5;
const DIAMETER_MM: f64 = 365.0;

pub fn standard() -> Flywheel {
    Flywheel::lightweight(MASS_KG, DIAMETER_MM)
}
