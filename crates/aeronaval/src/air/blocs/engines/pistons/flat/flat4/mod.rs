use crate::components::powertrain::engines::thermals::assemblies::flat::f4;

pub mod sn {
    pub use crate::components::powertrain::engines::thermals::assemblies::flat::f4::Displacement;
    pub struct D { pub displacement: Displacement, pub power_kw: f64, pub rpm_max: f64 }
    pub mod continental_o200_a { pub const D: super::D = super::D { displacement: super::Displacement { cc: 3_286, bore_mm: 103.2, stroke_mm:  98.4 }, power_kw:  74.6, rpm_max: 2_750.0 }; }
    pub mod lycoming_o320_b    { pub const D: super::D = super::D { displacement: super::Displacement { cc: 5_237, bore_mm: 130.2, stroke_mm:  98.4 }, power_kw: 112.0, rpm_max: 2_700.0 }; }
    pub mod lycoming_o360_a1a  { pub const D: super::D = super::D { displacement: super::Displacement { cc: 5_916, bore_mm: 130.2, stroke_mm: 111.1 }, power_kw: 134.0, rpm_max: 2_700.0 }; }
    pub mod rotax_912_uls      { pub const D: super::D = super::D { displacement: super::Displacement { cc: 1_211, bore_mm:  79.0, stroke_mm:  61.0 }, power_kw:  73.5, rpm_max: 5_800.0 }; }
    pub mod rotax_914          { pub const D: super::D = super::D { displacement: super::Displacement { cc: 1_211, bore_mm:  79.0, stroke_mm:  61.0 }, power_kw:  84.5, rpm_max: 5_800.0 }; }
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

pub fn cylinders() -> u8 { f4::CYLINDERS }
