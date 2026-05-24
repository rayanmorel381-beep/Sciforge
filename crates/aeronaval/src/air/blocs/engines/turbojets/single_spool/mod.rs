pub mod sn {
    pub struct D { pub thrust_kn: f64, pub pressure_ratio: f64 }
    pub mod jt8d_9 { pub const D: super::D = super::D { thrust_kn: 62.3, pressure_ratio: 16.9 }; }
    pub mod jt8d_217 { pub const D: super::D = super::D { thrust_kn: 89.0, pressure_ratio: 20.6 }; }
    pub mod jt3d_7 { pub const D: super::D = super::D { thrust_kn: 80.1, pressure_ratio: 13.6 }; }
    pub mod spey_511 { pub const D: super::D = super::D { thrust_kn: 54.0, pressure_ratio: 16.5 }; }
    pub mod olympus_593 { pub const D: super::D = super::D { thrust_kn: 140.0, pressure_ratio: 15.5 }; }
}

pub mod basic;
pub mod high_altitude;

use crate::components::powertrain::engines::turbines::assemblies::jet::turbojet::Turbojet;

pub fn all() -> Vec<Turbojet> {
    let mut v = Vec::new();
    v.extend(basic::all());
    v.extend(high_altitude::all());
    v
}
