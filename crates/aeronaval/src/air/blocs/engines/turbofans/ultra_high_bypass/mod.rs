pub mod sn {
    pub struct D { pub thrust_kn: f64, pub bypass_ratio: f64, pub fan_diameter_m: f64 }
    pub mod trent_900 { pub const D: super::D = super::D { thrust_kn: 340.0, bypass_ratio: 8.7,  fan_diameter_m: 2.946 }; }
    pub mod gp7270 { pub const D: super::D = super::D { thrust_kn: 311.4, bypass_ratio: 8.7,  fan_diameter_m: 2.946 }; }
    pub mod gp7277 { pub const D: super::D = super::D { thrust_kn: 340.0, bypass_ratio: 8.7,  fan_diameter_m: 2.946 }; }
    pub mod trent_970 { pub const D: super::D = super::D { thrust_kn: 374.0, bypass_ratio: 8.7,  fan_diameter_m: 2.946 }; }
    pub mod pw4168 { pub const D: super::D = super::D { thrust_kn: 302.5, bypass_ratio: 5.1,  fan_diameter_m: 2.540 }; }
    pub mod cf6_80e1a4 { pub const D: super::D = super::D { thrust_kn: 300.3, bypass_ratio: 5.8,  fan_diameter_m: 2.438 }; }
    pub mod leap_1c { pub const D: super::D = super::D { thrust_kn: 155.7, bypass_ratio: 11.0, fan_diameter_m: 1.980 }; }
    pub mod pw1400g { pub const D: super::D = super::D { thrust_kn: 155.7, bypass_ratio: 12.0, fan_diameter_m: 2.057 }; }
}

pub mod ducted;

use crate::components::powertrain::engines::turbines::assemblies::fan::turbofan::Turbofan;

pub fn all() -> Vec<Turbofan> {
    ducted::all()
}
