use crate::components::powertrain::engines::turbines::assemblies::propeller::{reduction_gear, turboprop::Turboprop};
use super::sn;

pub fn pt6a_67d() -> Turboprop { reduction_gear::high_altitude(sn::pt6a_67d::D.shaft_power_kw, sn::pt6a_67d::D.propeller_diameter_m, 16.0) }
pub fn pw127f() -> Turboprop { reduction_gear::high_altitude(sn::pw127f::D.shaft_power_kw, sn::pw127f::D.propeller_diameter_m, 19.7) }
pub fn pw127m() -> Turboprop { reduction_gear::high_altitude(sn::pw127m::D.shaft_power_kw, sn::pw127m::D.propeller_diameter_m, 19.7) }
pub fn pw150a() -> Turboprop { reduction_gear::high_altitude(sn::pw150a::D.shaft_power_kw, sn::pw150a::D.propeller_diameter_m, 20.0) }
pub fn ae2100d3() -> Turboprop { reduction_gear::high_altitude(sn::ae2100d3::D.shaft_power_kw, sn::ae2100d3::D.propeller_diameter_m, 14.4) }
pub fn tp400_d6() -> Turboprop { reduction_gear::high_altitude(sn::tp400_d6::D.shaft_power_kw, sn::tp400_d6::D.propeller_diameter_m, 8.79) }

pub fn all() -> Vec<Turboprop> {
    vec![pt6a_67d(), pw127f(), pw127m(), pw150a(), ae2100d3(), tp400_d6()]
}
