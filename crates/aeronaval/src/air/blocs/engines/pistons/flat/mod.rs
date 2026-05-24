pub mod flat4;
pub mod flat6;

use super::PistonEngine;

pub fn all() -> Vec<PistonEngine> {
    let mut v = Vec::new();
    v.extend(flat4::all());
    v.extend(flat6::all());
    v
}
