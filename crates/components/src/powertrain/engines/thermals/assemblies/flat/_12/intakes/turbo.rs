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
        compressor_inducer_mm: 78.0,
        turbine_exducer_mm: 82.0,
        max_boost_bar: 1.0,
    }
}

pub fn twin_scroll() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 82.0,
        turbine_exducer_mm: 86.0,
        max_boost_bar: 1.2,
    }
}

pub fn variable_geometry() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 84.0,
        turbine_exducer_mm: 88.0,
        max_boost_bar: 1.4,
    }
}

pub fn twin_parallel() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 64.0,
        turbine_exducer_mm: 68.0,
        max_boost_bar: 1.1,
    }
}

pub fn compression_ratio_low_boost() -> f64 {
    10.4
}

pub fn compression_ratio_standard() -> f64 {
    9.4
}

pub fn compression_ratio_high_boost() -> f64 {
    8.4
}
