use crate::components::powertrain::engines::turbines::assemblies::jet::{twin_spool, turbojet::Turbojet};
use super::sn;

pub fn jt9d_7r4() -> Turbojet { twin_spool::standard(sn::jt9d_7r4::D.thrust_kn, sn::jt9d_7r4::D.pressure_ratio) }
pub fn jt9d_7j() -> Turbojet { twin_spool::standard(sn::jt9d_7j::D.thrust_kn, sn::jt9d_7j::D.pressure_ratio) }
pub fn cfm56_2a2() -> Turbojet { twin_spool::standard(sn::cfm56_2a2::D.thrust_kn, sn::cfm56_2a2::D.pressure_ratio) }
pub fn cfm56_2c1() -> Turbojet { twin_spool::standard(sn::cfm56_2c1::D.thrust_kn, sn::cfm56_2c1::D.pressure_ratio) }
pub fn d_30kp_2() -> Turbojet { twin_spool::standard(sn::d_30kp_2::D.thrust_kn, sn::d_30kp_2::D.pressure_ratio) }
pub fn d_30ku_154() -> Turbojet { twin_spool::standard(sn::d_30ku_154::D.thrust_kn, sn::d_30ku_154::D.pressure_ratio) }
pub fn rb211_22b() -> Turbojet { twin_spool::standard(sn::rb211_22b::D.thrust_kn, sn::rb211_22b::D.pressure_ratio) }

pub fn all() -> Vec<Turbojet> {
    vec![jt9d_7r4(), jt9d_7j(), cfm56_2a2(), cfm56_2c1(), d_30kp_2(), d_30ku_154(), rb211_22b()]
}
