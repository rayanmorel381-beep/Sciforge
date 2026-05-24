use crate::components::powertrain::engines::turbines::assemblies::fan::{low_bypass, turbofan::Turbofan};
use super::sn;

pub fn f100_pw_220() -> Turbofan { low_bypass::afterburning(sn::f100_pw_220::D.thrust_kn, sn::f100_pw_220::D.bypass_ratio) }
pub fn f100_pw_229() -> Turbofan { low_bypass::afterburning(sn::f100_pw_229::D.thrust_kn, sn::f100_pw_229::D.bypass_ratio) }
pub fn f110_ge_100() -> Turbofan { low_bypass::afterburning(sn::f110_ge_100::D.thrust_kn, sn::f110_ge_100::D.bypass_ratio) }
pub fn f110_ge_129() -> Turbofan { low_bypass::afterburning(sn::f110_ge_129::D.thrust_kn, sn::f110_ge_129::D.bypass_ratio) }
pub fn f110_ge_132() -> Turbofan { low_bypass::afterburning(sn::f110_ge_132::D.thrust_kn, sn::f110_ge_132::D.bypass_ratio) }
pub fn al_31f() -> Turbofan { low_bypass::afterburning(sn::al_31f::D.thrust_kn, sn::al_31f::D.bypass_ratio) }
pub fn m88_2() -> Turbofan { low_bypass::afterburning(sn::m88_2::D.thrust_kn, sn::m88_2::D.bypass_ratio) }
pub fn f404_ge_402() -> Turbofan { low_bypass::afterburning(sn::f404_ge_402::D.thrust_kn, sn::f404_ge_402::D.bypass_ratio) }
pub fn f414_ge_400() -> Turbofan { low_bypass::afterburning(sn::f414_ge_400::D.thrust_kn, sn::f414_ge_400::D.bypass_ratio) }
pub fn rm12() -> Turbofan { low_bypass::afterburning(sn::rm12::D.thrust_kn, sn::rm12::D.bypass_ratio) }

pub fn all() -> Vec<Turbofan> {
    vec![f100_pw_220(), f100_pw_229(), f110_ge_100(), f110_ge_129(), f110_ge_132(), al_31f(), m88_2(), f404_ge_402(), f414_ge_400(), rm12()]
}
