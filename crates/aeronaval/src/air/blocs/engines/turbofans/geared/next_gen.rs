use crate::components::powertrain::engines::turbines::assemblies::fan::{geared, turbofan::Turbofan};
use super::sn;

pub fn leap_1a26() -> Turbofan { geared::next_gen(sn::leap_1a26::D.thrust_kn, sn::leap_1a26::D.bypass_ratio, sn::leap_1a26::D.fan_diameter_m) }
pub fn leap_1b27() -> Turbofan { geared::next_gen(sn::leap_1b27::D.thrust_kn, sn::leap_1b27::D.bypass_ratio, sn::leap_1b27::D.fan_diameter_m) }
pub fn leap_1c35() -> Turbofan { geared::next_gen(sn::leap_1c35::D.thrust_kn, sn::leap_1c35::D.bypass_ratio, sn::leap_1c35::D.fan_diameter_m) }
pub fn pw1100g_jm() -> Turbofan { geared::next_gen(sn::pw1100g_jm::D.thrust_kn, sn::pw1100g_jm::D.bypass_ratio, sn::pw1100g_jm::D.fan_diameter_m) }
pub fn pw1127g() -> Turbofan { geared::next_gen(sn::pw1127g::D.thrust_kn, sn::pw1127g::D.bypass_ratio, sn::pw1127g::D.fan_diameter_m) }
pub fn pw1133g() -> Turbofan { geared::next_gen(sn::pw1133g::D.thrust_kn, sn::pw1133g::D.bypass_ratio, sn::pw1133g::D.fan_diameter_m) }
pub fn pw1900g() -> Turbofan { geared::next_gen(sn::pw1900g::D.thrust_kn, sn::pw1900g::D.bypass_ratio, sn::pw1900g::D.fan_diameter_m) }
pub fn trent_7000() -> Turbofan { geared::next_gen(sn::trent_7000::D.thrust_kn, sn::trent_7000::D.bypass_ratio, sn::trent_7000::D.fan_diameter_m) }

pub fn all() -> Vec<Turbofan> {
    vec![leap_1a26(), leap_1b27(), leap_1c35(), pw1100g_jm(), pw1127g(), pw1133g(), pw1900g(), trent_7000()]
}
