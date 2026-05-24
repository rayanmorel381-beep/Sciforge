use crate::components::powertrain::engines::turbines::assemblies::jet::{postcombustion, turbojet::Turbojet};
use super::sn;

pub fn f100_pw_200() -> Turbojet { postcombustion::fighter(sn::f100_pw_200::D.dry_kn, sn::f100_pw_200::D.wet_kn, sn::f100_pw_200::D.pressure_ratio) }
pub fn f110_ge_400() -> Turbojet { postcombustion::fighter(sn::f110_ge_400::D.dry_kn, sn::f110_ge_400::D.wet_kn, sn::f110_ge_400::D.pressure_ratio) }
pub fn m53_p2() -> Turbojet { postcombustion::fighter(sn::m53_p2::D.dry_kn, sn::m53_p2::D.wet_kn, sn::m53_p2::D.pressure_ratio) }
pub fn rb199_mk104() -> Turbojet { postcombustion::fighter(sn::rb199_mk104::D.dry_kn, sn::rb199_mk104::D.wet_kn, sn::rb199_mk104::D.pressure_ratio) }
pub fn ws_10a() -> Turbojet { postcombustion::fighter(sn::ws_10a::D.dry_kn, sn::ws_10a::D.wet_kn, sn::ws_10a::D.pressure_ratio) }
pub fn snecma_m88_4e() -> Turbojet { postcombustion::fighter(sn::snecma_m88_4e::D.dry_kn, sn::snecma_m88_4e::D.wet_kn, sn::snecma_m88_4e::D.pressure_ratio) }

pub fn all() -> Vec<Turbojet> {
    vec![f100_pw_200(), f110_ge_400(), m53_p2(), rb199_mk104(), ws_10a(), snecma_m88_4e()]
}
