pub mod single_spool;
pub mod twin_spool;
pub mod postcombustion;

use crate::components::powertrain::engines::turbines::assemblies::jet::turbojet::Turbojet;

pub fn all() -> Vec<Turbojet> {
    let mut v = Vec::new();
    v.extend(single_spool::all());
    v.extend(twin_spool::all());
    v.extend(postcombustion::all());
    v
}
