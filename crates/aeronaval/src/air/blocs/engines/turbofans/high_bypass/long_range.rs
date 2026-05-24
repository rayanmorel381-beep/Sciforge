use crate::components::powertrain::engines::turbines::assemblies::fan::{high_bypass, turbofan::Turbofan};
use super::sn;

pub fn ge90_76b() -> Turbofan { high_bypass::long_range(sn::ge90_76b::D.thrust_kn, sn::ge90_76b::D.bypass_ratio, sn::ge90_76b::D.fan_diameter_m) }
pub fn ge90_115b() -> Turbofan { high_bypass::long_range(sn::ge90_115b::D.thrust_kn, sn::ge90_115b::D.bypass_ratio, sn::ge90_115b::D.fan_diameter_m) }
pub fn trent_700() -> Turbofan { high_bypass::long_range(sn::trent_700::D.thrust_kn, sn::trent_700::D.bypass_ratio, sn::trent_700::D.fan_diameter_m) }
pub fn trent_800() -> Turbofan { high_bypass::long_range(sn::trent_800::D.thrust_kn, sn::trent_800::D.bypass_ratio, sn::trent_800::D.fan_diameter_m) }
pub fn trent_1000() -> Turbofan { high_bypass::long_range(sn::trent_1000::D.thrust_kn, sn::trent_1000::D.bypass_ratio, sn::trent_1000::D.fan_diameter_m) }
pub fn pw4056() -> Turbofan { high_bypass::long_range(sn::pw4056::D.thrust_kn, sn::pw4056::D.bypass_ratio, sn::pw4056::D.fan_diameter_m) }
pub fn pw4090() -> Turbofan { high_bypass::long_range(sn::pw4090::D.thrust_kn, sn::pw4090::D.bypass_ratio, sn::pw4090::D.fan_diameter_m) }
pub fn genx_1b76() -> Turbofan { high_bypass::long_range(sn::genx_1b76::D.thrust_kn, sn::genx_1b76::D.bypass_ratio, sn::genx_1b76::D.fan_diameter_m) }
pub fn genx_2b67() -> Turbofan { high_bypass::long_range(sn::genx_2b67::D.thrust_kn, sn::genx_2b67::D.bypass_ratio, sn::genx_2b67::D.fan_diameter_m) }
pub fn trent_xwb_84() -> Turbofan { high_bypass::long_range(sn::trent_xwb_84::D.thrust_kn, sn::trent_xwb_84::D.bypass_ratio, sn::trent_xwb_84::D.fan_diameter_m) }
pub fn trent_xwb_97() -> Turbofan { high_bypass::long_range(sn::trent_xwb_97::D.thrust_kn, sn::trent_xwb_97::D.bypass_ratio, sn::trent_xwb_97::D.fan_diameter_m) }

pub fn all() -> Vec<Turbofan> {
    vec![ge90_76b(), ge90_115b(), trent_700(), trent_800(), trent_1000(), pw4056(), pw4090(), genx_1b76(), genx_2b67(), trent_xwb_84(), trent_xwb_97()]
}
