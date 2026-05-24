use crate::powertrain::engines::thermals::parts::flywheels::Flywheel;

const MASS_KG: f64 = 60.5;
const DIAMETER_MM: f64 = 460.0;

pub fn standard() -> Flywheel {
    Flywheel::dual_mass(MASS_KG, DIAMETER_MM)
}
