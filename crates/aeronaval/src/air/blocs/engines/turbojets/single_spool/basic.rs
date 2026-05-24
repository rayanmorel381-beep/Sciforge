use crate::components::powertrain::engines::turbines::assemblies::jet::{single_spool, turbojet::Turbojet};
use super::sn;

pub fn jt8d_9() -> Turbojet { single_spool::basic(sn::jt8d_9::D.thrust_kn, sn::jt8d_9::D.pressure_ratio) }
pub fn jt8d_217() -> Turbojet { single_spool::basic(sn::jt8d_217::D.thrust_kn, sn::jt8d_217::D.pressure_ratio) }
pub fn spey_511() -> Turbojet { single_spool::basic(sn::spey_511::D.thrust_kn, sn::spey_511::D.pressure_ratio) }

pub fn all() -> Vec<Turbojet> {
    vec![jt8d_9(), jt8d_217(), spey_511()]
}
