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
        compressor_inducer_mm: 90.0,
        turbine_exducer_mm: 95.0,
        max_boost_bar: 0.6,
    }
}

pub fn twin_scroll() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 100.0,
        turbine_exducer_mm: 108.0,
        max_boost_bar: 0.9,
    }
}

pub fn variable_geometry() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 105.0,
        turbine_exducer_mm: 112.0,
        max_boost_bar: 1.1,
    }
}

pub fn twin_parallel() -> Turbocharger {
    Turbocharger {
        compressor_inducer_mm: 80.0,
        turbine_exducer_mm: 86.0,
        max_boost_bar: 0.7,
    }
}

pub fn compression_ratio_low_boost() -> f64 {
    6.0
}

pub fn compression_ratio_standard() -> f64 {
    6.5
}

pub fn compression_ratio_high_boost() -> f64 {
    5.5
}
