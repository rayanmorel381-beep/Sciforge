use crate::components::powertrain::engines::turbines::assemblies::jet::{twin_spool, turbojet::Turbojet};
use super::sn;

pub fn ps_90a() -> Turbojet { twin_spool::high_performance(sn::ps_90a::D.thrust_kn, sn::ps_90a::D.pressure_ratio) }
pub fn rb211_524b() -> Turbojet { twin_spool::high_performance(sn::rb211_524b::D.thrust_kn, sn::rb211_524b::D.pressure_ratio) }
pub fn rb211_535e4() -> Turbojet { twin_spool::high_performance(sn::rb211_535e4::D.thrust_kn, sn::rb211_535e4::D.pressure_ratio) }

pub fn all() -> Vec<Turbojet> {
    vec![ps_90a(), rb211_524b(), rb211_535e4()]
}
