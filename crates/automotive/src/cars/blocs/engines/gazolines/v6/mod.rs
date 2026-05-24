pub use crate::components::powertrain::engines::thermals::assemblies::v::v6::cc;

pub mod biturbo;
pub mod na;
pub mod supercharged;
pub mod turbo;

use crate::components::powertrain::engines::thermals::assemblies::bloc::Bloc;

pub fn all() -> Vec<Bloc> {
    let mut v = Vec::new();
    v.extend(na::all());
    v.extend(turbo::all());
    v.extend(biturbo::all());
    v.extend(supercharged::all());
    v
}
