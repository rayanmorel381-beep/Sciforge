use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

const MASS_KG: f64 = 18.0;
const DIAMETER_MM: f64 = 270.0;

pub fn standard() -> Flywheel {
    Flywheel::dual_mass(MASS_KG, DIAMETER_MM)
}
