pub mod free_turbine;
pub mod reduction_gear;

use crate::components::powertrain::engines::turbines::assemblies::propeller::turboprop::Turboprop;

pub fn all() -> Vec<Turboprop> {
    let mut v = Vec::new();
    v.extend(free_turbine::all());
    v.extend(reduction_gear::all());
    v
}
