use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

const MASS_KG: f64 = 13.5;
const DIAMETER_MM: f64 = 310.0;

pub fn standard() -> Flywheel {
    Flywheel::lightweight(MASS_KG, DIAMETER_MM)
}
