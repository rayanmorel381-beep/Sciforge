use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

const MASS_KG: f64 = 26.0;
const DIAMETER_MM: f64 = 305.0;

pub fn standard() -> Flywheel {
    Flywheel::dual_mass(MASS_KG, DIAMETER_MM)
}
