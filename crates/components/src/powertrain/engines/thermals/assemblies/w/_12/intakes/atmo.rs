use crate::powertrain::engines::thermals::parts::intake::IntakeSystem;

use super::super::Displacement;

pub fn standard(d: &Displacement) -> IntakeSystem {
    IntakeSystem::naturally_aspirated(d.liters())
}

pub fn high_compression(d: &Displacement) -> IntakeSystem {
    IntakeSystem {
        variable_geometry: true,
        resonance_chamber: true,
        ..IntakeSystem::naturally_aspirated(d.liters())
    }
}

pub fn compression_ratio() -> f64 {
    11.3
}

pub fn high_compression_ratio() -> f64 {
    13.2
}
