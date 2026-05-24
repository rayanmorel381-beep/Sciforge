use crate::powertrain::engines::thermals::parts::intake::IntakeSystem;
use crate::powertrain::engines::thermals::parts::turbos::Turbocharger;

use super::super::Displacement;

pub fn intake_quad(d: &Displacement) -> IntakeSystem {
    IntakeSystem::twin_turbo(d.liters())
}

pub fn quad_parallel() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 60.0,
        turbine_exducer_mm: 64.0,
        max_boost_bar: 1.55,
    }
}

pub fn quad_sequential() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 64.0,
        turbine_exducer_mm: 68.0,
        max_boost_bar: 1.7,
    }
}

pub fn compression_ratio_standard() -> f64 {
    9.0
}
