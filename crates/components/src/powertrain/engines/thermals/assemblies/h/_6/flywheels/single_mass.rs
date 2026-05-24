use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

const MASS_KG: f64 = 11.0;
const DIAMETER_MM: f64 = 255.0;

pub fn standard() -> Flywheel {
    Flywheel::single_mass(MASS_KG, DIAMETER_MM)
}
