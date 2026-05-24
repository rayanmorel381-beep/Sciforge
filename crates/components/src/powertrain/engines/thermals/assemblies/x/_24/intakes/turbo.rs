use crate::powertrain::engines::thermals::parts::intake::IntakeSystem;
use crate::powertrain::engines::thermals::parts::turbos::Turbocharger;

use super::super::Displacement;

pub fn intake_twin(d: &Displacement) -> IntakeSystem {
    IntakeSystem::twin_turbo(d.liters())
}

pub fn twin_scroll() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 80.0,
        turbine_exducer_mm: 84.0,
        max_boost_bar: 1.4,
    }
}

pub fn compression_ratio_standard() -> f64 {
    9.5
}
