use crate::powertrain::engines::thermals::parts::compressors::Supercharger;
use crate::powertrain::engines::thermals::parts::intake::IntakeSystem;

use super::super::Displacement;

pub fn intake(d: &Displacement) -> IntakeSystem {
    IntakeSystem::supercharged(d.liters())
}

pub fn roots(d: &Displacement) -> Supercharger {
    Supercharger::roots(d.cc)
}

pub fn twin_screw(d: &Displacement) -> Supercharger {
    Supercharger::twin_screw(d.cc, true)
}

pub fn centrifugal(d: &Displacement) -> Supercharger {
    Supercharger::centrifugal(d.cc)
}

pub fn compression_ratio() -> f64 {
    10.0
}
