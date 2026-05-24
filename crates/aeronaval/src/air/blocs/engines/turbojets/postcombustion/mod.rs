pub mod sn {
    pub struct D { pub dry_kn: f64, pub wet_kn: f64, pub pressure_ratio: f64 }
    pub mod f100_pw_200 { pub const D: super::D = super::D { dry_kn:  64.9, wet_kn: 105.7, pressure_ratio: 25.0 }; }
    pub mod f110_ge_400 { pub const D: super::D = super::D { dry_kn:  75.6, wet_kn: 128.9, pressure_ratio: 30.4 }; }
    pub mod al_41f1 { pub const D: super::D = super::D { dry_kn:  93.1, wet_kn: 147.1, pressure_ratio: 26.0 }; }
    pub mod al_51f { pub const D: super::D = super::D { dry_kn: 107.9, wet_kn: 175.0, pressure_ratio: 28.0 }; }
    pub mod m53_p2 { pub const D: super::D = super::D { dry_kn:  64.3, wet_kn:  95.1, pressure_ratio: 9.8 }; }
    pub mod rb199_mk104 { pub const D: super::D = super::D { dry_kn:  43.8, wet_kn:  76.8, pressure_ratio: 23.5 }; }
    pub mod f119_pw_100 { pub const D: super::D = super::D { dry_kn: 116.0, wet_kn: 156.0, pressure_ratio: 26.0 }; }
    pub mod f135_pw_100 { pub const D: super::D = super::D { dry_kn: 124.5, wet_kn: 191.3, pressure_ratio: 28.0 }; }
    pub mod ws_10a { pub const D: super::D = super::D { dry_kn:  76.3, wet_kn: 132.0, pressure_ratio: 30.0 }; }
    pub mod ws_15 { pub const D: super::D = super::D { dry_kn: 102.0, wet_kn: 147.0, pressure_ratio: 32.0 }; }
    pub mod snecma_m88_4e { pub const D: super::D = super::D { dry_kn:  52.5, wet_kn:  90.0, pressure_ratio: 27.0 }; }
    pub mod saturn_al_31fm { pub const D: super::D = super::D { dry_kn:  86.3, wet_kn: 142.2, pressure_ratio: 22.1 }; }
}

pub mod fighter;
pub mod interceptor;

use crate::components::powertrain::engines::turbines::assemblies::jet::turbojet::Turbojet;

pub fn all() -> Vec<Turbojet> {
    let mut v = Vec::new();
    v.extend(fighter::all());
    v.extend(interceptor::all());
    v
}
