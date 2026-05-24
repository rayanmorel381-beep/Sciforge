use crate::components::powertrain::engines::turbines::assemblies::jet::{single_spool, turbojet::Turbojet};
use super::sn;

pub fn jt3d_7() -> Turbojet { single_spool::high_altitude(sn::jt3d_7::D.thrust_kn, sn::jt3d_7::D.pressure_ratio) }
pub fn olympus_593() -> Turbojet { single_spool::high_altitude(sn::olympus_593::D.thrust_kn, sn::olympus_593::D.pressure_ratio) }

pub fn all() -> Vec<Turbojet> {
    vec![jt3d_7(), olympus_593()]
}
