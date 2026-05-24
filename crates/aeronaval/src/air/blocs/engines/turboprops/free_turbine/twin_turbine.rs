use crate::components::powertrain::engines::turbines::assemblies::propeller::{free_turbine, turboprop::Turboprop};
use super::sn;

pub fn t56_a_15() -> Turboprop { free_turbine::twin_turbine(sn::t56_a_15::D.shaft_power_kw, sn::t56_a_15::D.propeller_diameter_m) }
pub fn t56_a_16() -> Turboprop { free_turbine::twin_turbine(sn::t56_a_16::D.shaft_power_kw, sn::t56_a_16::D.propeller_diameter_m) }
pub fn rtm322_01() -> Turboprop { free_turbine::twin_turbine(sn::rtm322_01::D.shaft_power_kw, sn::rtm322_01::D.propeller_diameter_m) }
pub fn ai_20m() -> Turboprop { free_turbine::twin_turbine(sn::ai_20m::D.shaft_power_kw, sn::ai_20m::D.propeller_diameter_m) }
pub fn nk_12ma() -> Turboprop { free_turbine::twin_turbine(sn::nk_12ma::D.shaft_power_kw, sn::nk_12ma::D.propeller_diameter_m) }
pub fn t64_ge_416() -> Turboprop { free_turbine::twin_turbine(sn::t64_ge_416::D.shaft_power_kw, sn::t64_ge_416::D.propeller_diameter_m) }

pub fn all() -> Vec<Turboprop> {
    vec![t56_a_15(), t56_a_16(), rtm322_01(), ai_20m(), nk_12ma(), t64_ge_416()]
}
