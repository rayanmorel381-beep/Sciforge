pub mod aluminum;
pub mod calcium;
pub mod copper;
pub mod lithium;
pub mod molybdenum;
pub mod polyurea;
pub mod sodium;
pub mod synthetic;


use crate::lubrications::Grease;

pub fn all_greases() -> Vec<&'static Grease> {
    let mut v = Vec::new();
    v.extend(lithium::all_lithium_greases());
    v.extend(calcium::all_calcium_greases());
    v.extend(molybdenum::all_molybdenum_greases());
    v.extend(polyurea::all_polyurea_greases());
    v.extend(aluminum::all_aluminum_greases());
    v.extend(sodium::all_sodium_greases());
    v.extend(synthetic::all_synthetic_greases());
    v.extend(copper::all_copper_greases());
    v
}

pub fn grease_by_name(name: &str) -> Option<&'static Grease> {
    all_greases().into_iter().find(|g| g.name == name)
}
