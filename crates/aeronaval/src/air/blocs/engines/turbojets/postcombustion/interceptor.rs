use crate::components::powertrain::engines::turbines::assemblies::jet::{postcombustion, turbojet::Turbojet};
use super::sn;

pub fn al_41f1() -> Turbojet { postcombustion::interceptor(sn::al_41f1::D.dry_kn, sn::al_41f1::D.wet_kn, sn::al_41f1::D.pressure_ratio) }
pub fn al_51f() -> Turbojet { postcombustion::interceptor(sn::al_51f::D.dry_kn, sn::al_51f::D.wet_kn, sn::al_51f::D.pressure_ratio) }
pub fn f119_pw_100() -> Turbojet { postcombustion::interceptor(sn::f119_pw_100::D.dry_kn, sn::f119_pw_100::D.wet_kn, sn::f119_pw_100::D.pressure_ratio) }
pub fn f135_pw_100() -> Turbojet { postcombustion::interceptor(sn::f135_pw_100::D.dry_kn, sn::f135_pw_100::D.wet_kn, sn::f135_pw_100::D.pressure_ratio) }
pub fn ws_15() -> Turbojet { postcombustion::interceptor(sn::ws_15::D.dry_kn, sn::ws_15::D.wet_kn, sn::ws_15::D.pressure_ratio) }
pub fn saturn_al_31fm() -> Turbojet { postcombustion::interceptor(sn::saturn_al_31fm::D.dry_kn, sn::saturn_al_31fm::D.wet_kn, sn::saturn_al_31fm::D.pressure_ratio) }

pub fn all() -> Vec<Turbojet> {
    vec![al_41f1(), al_51f(), f119_pw_100(), f135_pw_100(), ws_15(), saturn_al_31fm()]
}
