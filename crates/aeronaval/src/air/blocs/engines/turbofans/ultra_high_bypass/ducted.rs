use crate::components::powertrain::engines::turbines::assemblies::fan::{ultra_high_bypass, turbofan::Turbofan};
use super::sn;

pub fn trent_900() -> Turbofan { ultra_high_bypass::ducted(sn::trent_900::D.thrust_kn, sn::trent_900::D.bypass_ratio, sn::trent_900::D.fan_diameter_m) }
pub fn gp7270() -> Turbofan { ultra_high_bypass::ducted(sn::gp7270::D.thrust_kn, sn::gp7270::D.bypass_ratio, sn::gp7270::D.fan_diameter_m) }
pub fn gp7277() -> Turbofan { ultra_high_bypass::ducted(sn::gp7277::D.thrust_kn, sn::gp7277::D.bypass_ratio, sn::gp7277::D.fan_diameter_m) }
pub fn trent_970() -> Turbofan { ultra_high_bypass::ducted(sn::trent_970::D.thrust_kn, sn::trent_970::D.bypass_ratio, sn::trent_970::D.fan_diameter_m) }
pub fn pw4168() -> Turbofan { ultra_high_bypass::ducted(sn::pw4168::D.thrust_kn, sn::pw4168::D.bypass_ratio, sn::pw4168::D.fan_diameter_m) }
pub fn cf6_80e1a4() -> Turbofan { ultra_high_bypass::ducted(sn::cf6_80e1a4::D.thrust_kn, sn::cf6_80e1a4::D.bypass_ratio, sn::cf6_80e1a4::D.fan_diameter_m) }
pub fn leap_1c() -> Turbofan { ultra_high_bypass::ducted(sn::leap_1c::D.thrust_kn, sn::leap_1c::D.bypass_ratio, sn::leap_1c::D.fan_diameter_m) }
pub fn pw1400g() -> Turbofan { ultra_high_bypass::ducted(sn::pw1400g::D.thrust_kn, sn::pw1400g::D.bypass_ratio, sn::pw1400g::D.fan_diameter_m) }

pub fn all() -> Vec<Turbofan> {
    vec![trent_900(), gp7270(), gp7277(), trent_970(), pw4168(), cf6_80e1a4(), leap_1c(), pw1400g()]
}
