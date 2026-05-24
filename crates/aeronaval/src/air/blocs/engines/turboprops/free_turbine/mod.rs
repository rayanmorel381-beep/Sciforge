pub mod sn {
    pub struct D { pub shaft_power_kw: f64, pub propeller_diameter_m: f64 }
    pub mod t56_a_15 { pub const D: super::D = super::D { shaft_power_kw: 3_661.0, propeller_diameter_m: 4.11 }; }
    pub mod t56_a_16 { pub const D: super::D = super::D { shaft_power_kw: 3_661.0, propeller_diameter_m: 4.11 }; }
    pub mod rtm322_01 { pub const D: super::D = super::D { shaft_power_kw: 2_095.0, propeller_diameter_m: 3.20 }; }
    pub mod tv3_117vm { pub const D: super::D = super::D { shaft_power_kw: 1_640.0, propeller_diameter_m: 2.70 }; }
    pub mod ai_20m { pub const D: super::D = super::D { shaft_power_kw: 2_942.0, propeller_diameter_m: 4.50 }; }
    pub mod nk_12ma { pub const D: super::D = super::D { shaft_power_kw: 11_185.0, propeller_diameter_m: 5.60 }; }
    pub mod t64_ge_416 { pub const D: super::D = super::D { shaft_power_kw: 3_266.0, propeller_diameter_m: 4.00 }; }
}

pub mod twin_turbine;
pub mod single_turbine;

use crate::components::powertrain::engines::turbines::assemblies::propeller::turboprop::Turboprop;

pub fn all() -> Vec<Turboprop> {
    let mut v = Vec::new();
    v.extend(twin_turbine::all());
    v.extend(single_turbine::all());
    v
}
