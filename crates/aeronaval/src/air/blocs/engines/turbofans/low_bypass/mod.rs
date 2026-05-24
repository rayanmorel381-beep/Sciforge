pub mod sn {
    pub struct D { pub thrust_kn: f64, pub bypass_ratio: f64, pub fan_diameter_m: f64 }
    pub mod f100_pw_220 { pub const D: super::D = super::D { thrust_kn:  65.3, bypass_ratio: 0.36, fan_diameter_m: 0.884 }; }
    pub mod f100_pw_229 { pub const D: super::D = super::D { thrust_kn:  79.2, bypass_ratio: 0.36, fan_diameter_m: 0.884 }; }
    pub mod f110_ge_100 { pub const D: super::D = super::D { thrust_kn:  76.3, bypass_ratio: 0.87, fan_diameter_m: 0.958 }; }
    pub mod f110_ge_129 { pub const D: super::D = super::D { thrust_kn:  76.3, bypass_ratio: 0.76, fan_diameter_m: 0.958 }; }
    pub mod f110_ge_132 { pub const D: super::D = super::D { thrust_kn:  86.7, bypass_ratio: 0.68, fan_diameter_m: 0.958 }; }
    pub mod al_31f { pub const D: super::D = super::D { thrust_kn:  74.5, bypass_ratio: 0.59, fan_diameter_m: 0.905 }; }
    pub mod rd_93 { pub const D: super::D = super::D { thrust_kn:  49.4, bypass_ratio: 0.49, fan_diameter_m: 0.670 }; }
    pub mod m88_2 { pub const D: super::D = super::D { thrust_kn:  50.0, bypass_ratio: 0.30, fan_diameter_m: 0.696 }; }
    pub mod ej200 { pub const D: super::D = super::D { thrust_kn:  60.0, bypass_ratio: 0.40, fan_diameter_m: 0.740 }; }
    pub mod f404_ge_402 { pub const D: super::D = super::D { thrust_kn:  53.0, bypass_ratio: 0.34, fan_diameter_m: 0.787 }; }
    pub mod f414_ge_400 { pub const D: super::D = super::D { thrust_kn:  57.8, bypass_ratio: 0.30, fan_diameter_m: 0.787 }; }
    pub mod rm12 { pub const D: super::D = super::D { thrust_kn:  54.0, bypass_ratio: 0.31, fan_diameter_m: 0.787 }; }
}

pub mod military;
pub mod afterburning;

use crate::components::powertrain::engines::turbines::assemblies::fan::turbofan::Turbofan;

pub fn all() -> Vec<Turbofan> {
    let mut v = Vec::new();
    v.extend(military::all());
    v.extend(afterburning::all());
    v
}
