pub mod sn {
    pub struct D { pub thrust_kn: f64, pub pressure_ratio: f64 }
    pub mod jt9d_7r4 { pub const D: super::D = super::D { thrust_kn: 218.0, pressure_ratio: 24.5 }; }
    pub mod jt9d_7j { pub const D: super::D = super::D { thrust_kn: 236.0, pressure_ratio: 25.0 }; }
    pub mod cfm56_2a2 { pub const D: super::D = super::D { thrust_kn: 97.9, pressure_ratio: 32.0 }; }
    pub mod cfm56_2c1 { pub const D: super::D = super::D { thrust_kn: 97.9, pressure_ratio: 32.0 }; }
    pub mod d_30kp_2 { pub const D: super::D = super::D { thrust_kn: 117.7, pressure_ratio: 20.0 }; }
    pub mod d_30ku_154 { pub const D: super::D = super::D { thrust_kn: 104.0, pressure_ratio: 18.5 }; }
    pub mod ps_90a { pub const D: super::D = super::D { thrust_kn: 157.0, pressure_ratio: 35.5 }; }
    pub mod rb211_22b { pub const D: super::D = super::D { thrust_kn: 166.4, pressure_ratio: 25.0 }; }
    pub mod rb211_524b { pub const D: super::D = super::D { thrust_kn: 222.4, pressure_ratio: 28.0 }; }
    pub mod rb211_535e4 { pub const D: super::D = super::D { thrust_kn: 178.8, pressure_ratio: 28.0 }; }
}

pub mod standard;
pub mod high_performance;

use crate::components::powertrain::engines::turbines::assemblies::jet::turbojet::Turbojet;

pub fn all() -> Vec<Turbojet> {
    let mut v = Vec::new();
    v.extend(standard::all());
    v.extend(high_performance::all());
    v
}
