use crate::powertrain::engines::thermals::parts::intake::IntakeSystem;
use crate::powertrain::engines::thermals::parts::turbos::Turbocharger;

use super::super::Displacement;

pub fn intake_twin(d: &Displacement) -> IntakeSystem {
    IntakeSystem::twin_turbo(d.liters())
}

pub fn twin_scroll() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 78.0,
        turbine_exducer_mm: 82.0,
        max_boost_bar: 1.4,
    }
}

pub fn quad_parallel() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 58.0,
        turbine_exducer_mm: 62.0,
        max_boost_bar: 1.3,
    }
}

pub fn compression_ratio_standard() -> f64 {
    9.5
}
