use crate::powertrain::engines::thermals::parts::intake::IntakeSystem;
use crate::powertrain::engines::thermals::parts::turbos::Turbocharger;

use super::super::Displacement;

pub fn intake_twin(d: &Displacement) -> IntakeSystem {
    IntakeSystem::twin_turbo(d.liters())
}

pub fn twin_scroll() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 88.0,
        turbine_exducer_mm: 92.0,
        max_boost_bar: 1.4,
    }
}

pub fn quad_parallel() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 66.0,
        turbine_exducer_mm: 70.0,
        max_boost_bar: 1.4,
    }
}

pub fn compression_ratio_standard() -> f64 {
    8.5
}
