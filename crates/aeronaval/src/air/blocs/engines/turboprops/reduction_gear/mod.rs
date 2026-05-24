pub mod sn {
    pub struct D { pub shaft_power_kw: f64, pub propeller_diameter_m: f64 }
    pub mod pt6a_27 { pub const D: super::D = super::D { shaft_power_kw:  507.0, propeller_diameter_m: 2.36 }; }
    pub mod pt6a_60a { pub const D: super::D = super::D { shaft_power_kw:  783.0, propeller_diameter_m: 2.44 }; }
    pub mod pt6a_67d { pub const D: super::D = super::D { shaft_power_kw:  895.0, propeller_diameter_m: 2.54 }; }
    pub mod tpe331_10 { pub const D: super::D = super::D { shaft_power_kw:  746.0, propeller_diameter_m: 2.44 }; }
    pub mod tpe331_12ub { pub const D: super::D = super::D { shaft_power_kw:  746.0, propeller_diameter_m: 2.54 }; }
    pub mod pw127f { pub const D: super::D = super::D { shaft_power_kw: 2_051.0, propeller_diameter_m: 3.96 }; }
    pub mod pw127m { pub const D: super::D = super::D { shaft_power_kw: 2_051.0, propeller_diameter_m: 3.96 }; }
    pub mod pw150a { pub const D: super::D = super::D { shaft_power_kw: 3_781.0, propeller_diameter_m: 4.11 }; }
    pub mod ct7_9b { pub const D: super::D = super::D { shaft_power_kw: 1_305.0, propeller_diameter_m: 2.90 }; }
    pub mod ae2100d3 { pub const D: super::D = super::D { shaft_power_kw: 3_458.0, propeller_diameter_m: 4.11 }; }
    pub mod tp400_d6 { pub const D: super::D = super::D { shaft_power_kw: 8_203.0, propeller_diameter_m: 5.33 }; }
    pub mod tvd_10b { pub const D: super::D = super::D { shaft_power_kw:  820.0, propeller_diameter_m: 2.70 }; }
}

use crate::components::powertrain::engines::turbines::assemblies::propeller::{reduction_gear, turboprop::Turboprop};

pub fn pt6a_27() -> Turboprop { reduction_gear::standard(sn::pt6a_27::D.shaft_power_kw, sn::pt6a_27::D.propeller_diameter_m, 15.0) }
pub fn pt6a_60a() -> Turboprop { reduction_gear::standard(sn::pt6a_60a::D.shaft_power_kw, sn::pt6a_60a::D.propeller_diameter_m, 15.0) }
pub fn pt6a_67d() -> Turboprop { reduction_gear::high_altitude(sn::pt6a_67d::D.shaft_power_kw, sn::pt6a_67d::D.propeller_diameter_m, 16.0) }
pub fn tpe331_10() -> Turboprop { reduction_gear::standard(sn::tpe331_10::D.shaft_power_kw, sn::tpe331_10::D.propeller_diameter_m, 12.5) }
pub fn tpe331_12ub() -> Turboprop { reduction_gear::standard(sn::tpe331_12ub::D.shaft_power_kw, sn::tpe331_12ub::D.propeller_diameter_m, 12.5) }
pub fn pw127f() -> Turboprop { reduction_gear::high_altitude(sn::pw127f::D.shaft_power_kw, sn::pw127f::D.propeller_diameter_m, 19.7) }
pub fn pw127m() -> Turboprop { reduction_gear::high_altitude(sn::pw127m::D.shaft_power_kw, sn::pw127m::D.propeller_diameter_m, 19.7) }
pub fn pw150a() -> Turboprop { reduction_gear::high_altitude(sn::pw150a::D.shaft_power_kw, sn::pw150a::D.propeller_diameter_m, 20.0) }
pub fn ct7_9b() -> Turboprop { reduction_gear::standard(sn::ct7_9b::D.shaft_power_kw, sn::ct7_9b::D.propeller_diameter_m, 15.5) }
pub fn ae2100d3() -> Turboprop { reduction_gear::high_altitude(sn::ae2100d3::D.shaft_power_kw, sn::ae2100d3::D.propeller_diameter_m, 14.4) }
pub fn tp400_d6() -> Turboprop { reduction_gear::high_altitude(sn::tp400_d6::D.shaft_power_kw, sn::tp400_d6::D.propeller_diameter_m, 8.79) }
pub fn tvd_10b() -> Turboprop { reduction_gear::standard(sn::tvd_10b::D.shaft_power_kw, sn::tvd_10b::D.propeller_diameter_m, 12.0) }

pub fn all() -> Vec<Turboprop> {
    vec![
        pt6a_27(), pt6a_60a(), pt6a_67d(),
        tpe331_10(), tpe331_12ub(),
        pw127f(), pw127m(), pw150a(),
        ct7_9b(), ae2100d3(), tp400_d6(),
        tvd_10b(),
    ]
}
