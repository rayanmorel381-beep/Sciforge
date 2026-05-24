use crate::components::powertrain::engines::thermals::assemblies::flat::f6;

pub mod sn {
    pub use crate::components::powertrain::engines::thermals::assemblies::flat::f6::Displacement;
    pub struct D { pub displacement: Displacement, pub power_kw: f64, pub rpm_max: f64 }
    pub mod lycoming_io540_k       { pub const D: super::D = super::D { displacement: super::Displacement { cc: 8_899, bore_mm: 130.2, stroke_mm: 111.1 }, power_kw: 224.0, rpm_max: 2_575.0 }; }
    pub mod continental_io520_ba   { pub const D: super::D = super::D { displacement: super::Displacement { cc: 8_523, bore_mm: 133.4, stroke_mm: 101.6 }, power_kw: 213.0, rpm_max: 2_700.0 }; }
    pub mod continental_io550_n    { pub const D: super::D = super::D { displacement: super::Displacement { cc: 9_013, bore_mm: 133.4, stroke_mm: 107.9 }, power_kw: 224.0, rpm_max: 2_700.0 }; }
    pub mod continental_tsio520_nb { pub const D: super::D = super::D { displacement: super::Displacement { cc: 8_523, bore_mm: 133.4, stroke_mm: 101.6 }, power_kw: 240.0, rpm_max: 2_700.0 }; }
}

pub mod aspirated;
pub mod turbocharged;

use super::super::PistonEngine;

pub fn all() -> Vec<PistonEngine> {
    let mut v = Vec::new();
    v.extend(aspirated::all());
    v.extend(turbocharged::all());
    v
}

pub fn cylinders() -> u8 { f6::CYLINDERS }
