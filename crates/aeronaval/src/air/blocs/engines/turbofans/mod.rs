pub mod high_bypass;
pub mod geared;
pub mod low_bypass;
pub mod ultra_high_bypass;

use crate::components::powertrain::engines::turbines::assemblies::fan::turbofan::Turbofan;

pub fn all() -> Vec<Turbofan> {
    let mut v = Vec::new();
    v.extend(high_bypass::all());
    v.extend(geared::all());
    v.extend(low_bypass::all());
    v.extend(ultra_high_bypass::all());
    v
}
