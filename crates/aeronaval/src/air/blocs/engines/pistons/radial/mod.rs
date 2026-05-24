pub mod sn {
    pub use crate::components::powertrain::engines::thermals::assemblies::radials::r7::Displacement as D7Displacement;
    pub use crate::components::powertrain::engines::thermals::assemblies::radials::r9::Displacement as D9Displacement;
    pub struct D7 { pub displacement: D7Displacement, pub power_kw: f64, pub rpm_max: f64 }
    pub struct D9 { pub displacement: D9Displacement, pub power_kw: f64, pub rpm_max: f64 }
    pub mod gnome_7_omega   { pub const D: super::D7 = super::D7 { displacement: super::D7Displacement { cc:  7_983, bore_mm: 110.0, stroke_mm: 120.0 }, power_kw:  37.3, rpm_max: 1_200.0 }; }
    pub mod gnome_7_lambda  { pub const D: super::D7 = super::D7 { displacement: super::D7Displacement { cc: 12_297, bore_mm: 120.0, stroke_mm: 155.0 }, power_kw:  59.7, rpm_max: 1_200.0 }; }
    pub mod wright_j5       { pub const D: super::D9 = super::D9 { displacement: super::D9Displacement { cc: 12_929, bore_mm: 114.3, stroke_mm: 139.7 }, power_kw: 164.1, rpm_max: 1_800.0 }; }
    pub mod pw_r985_wasp_jr { pub const D: super::D9 = super::D9 { displacement: super::D9Displacement { cc: 16_141, bore_mm: 133.4, stroke_mm: 130.2 }, power_kw: 335.6, rpm_max: 2_300.0 }; }
    pub mod pw_r1340_wasp   { pub const D: super::D9 = super::D9 { displacement: super::D9Displacement { cc: 21_990, bore_mm: 146.1, stroke_mm: 146.1 }, power_kw: 432.4, rpm_max: 2_250.0 }; }
    pub mod wright_r1820    { pub const D: super::D9 = super::D9 { displacement: super::D9Displacement { cc: 29_898, bore_mm: 155.6, stroke_mm: 174.6 }, power_kw: 671.1, rpm_max: 2_500.0 }; }
}

pub mod seven_cylinder;
pub mod nine_cylinder;

use super::PistonEngine;

pub fn all() -> Vec<PistonEngine> {
    let mut v = Vec::new();
    v.extend(seven_cylinder::all());
    v.extend(nine_cylinder::all());
    v
}
