pub mod sn {
    pub struct D { pub thrust_kn: f64, pub bypass_ratio: f64, pub fan_diameter_m: f64 }
    pub mod leap_1a26 { pub const D: super::D = super::D { thrust_kn: 120.1, bypass_ratio: 11.0, fan_diameter_m: 1.980 }; }
    pub mod leap_1b27 { pub const D: super::D = super::D { thrust_kn: 120.1, bypass_ratio: 9.0,  fan_diameter_m: 1.762 }; }
    pub mod leap_1c35 { pub const D: super::D = super::D { thrust_kn: 155.7, bypass_ratio: 9.0,  fan_diameter_m: 1.980 }; }
    pub mod pw1100g_jm { pub const D: super::D = super::D { thrust_kn: 120.1, bypass_ratio: 12.0, fan_diameter_m: 2.057 }; }
    pub mod pw1127g { pub const D: super::D = super::D { thrust_kn: 120.1, bypass_ratio: 12.0, fan_diameter_m: 2.057 }; }
    pub mod pw1133g { pub const D: super::D = super::D { thrust_kn: 147.0, bypass_ratio: 12.5, fan_diameter_m: 2.057 }; }
    pub mod pw1500g { pub const D: super::D = super::D { thrust_kn:  86.3, bypass_ratio: 12.0, fan_diameter_m: 1.701 }; }
    pub mod pw1900g { pub const D: super::D = super::D { thrust_kn: 147.0, bypass_ratio: 12.0, fan_diameter_m: 2.057 }; }
    pub mod trent_7000 { pub const D: super::D = super::D { thrust_kn: 320.0, bypass_ratio: 10.0, fan_diameter_m: 2.845 }; }
}

pub mod next_gen;
pub mod regional;

use crate::components::powertrain::engines::turbines::assemblies::fan::turbofan::Turbofan;

pub fn all() -> Vec<Turbofan> {
    let mut v = Vec::new();
    v.extend(next_gen::all());
    v.extend(regional::all());
    v
}
