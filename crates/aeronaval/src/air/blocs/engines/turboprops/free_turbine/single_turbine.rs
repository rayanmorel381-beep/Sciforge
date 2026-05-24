use crate::components::powertrain::engines::turbines::assemblies::propeller::{free_turbine, turboprop::Turboprop};
use super::sn;

pub fn tv3_117vm() -> Turboprop { free_turbine::single_turbine(sn::tv3_117vm::D.shaft_power_kw, sn::tv3_117vm::D.propeller_diameter_m) }

pub fn all() -> Vec<Turboprop> {
    vec![tv3_117vm()]
}
