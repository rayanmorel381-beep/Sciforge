use crate::powertrain::engines::thermals::parts::intake::IntakeSystem;
use crate::powertrain::engines::thermals::parts::turbos::Turbocharger;

use super::super::Displacement;

pub fn intake_single(d: &Displacement) -> IntakeSystem {
    IntakeSystem::turbocharged(d.liters())
}

pub fn intake_twin(d: &Displacement) -> IntakeSystem {
    IntakeSystem::twin_turbo(d.liters())
}

pub fn single_scroll() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 60.0,
        turbine_exducer_mm: 64.0,
        max_boost_bar: 1.2,
    }
}

pub fn twin_scroll() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 64.0,
        turbine_exducer_mm: 68.0,
        max_boost_bar: 1.5,
    }
}

pub fn variable_geometry() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 66.0,
        turbine_exducer_mm: 70.0,
        max_boost_bar: 1.7,
    }
}

pub fn twin_parallel() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 52.0,
        turbine_exducer_mm: 56.0,
        max_boost_bar: 1.4,
    }
}

pub fn compression_ratio_low_boost() -> f64 {
    9.8
}

pub fn compression_ratio_standard() -> f64 {
    10.0
}

pub fn compression_ratio_high_boost() -> f64 {
    9.0
}
