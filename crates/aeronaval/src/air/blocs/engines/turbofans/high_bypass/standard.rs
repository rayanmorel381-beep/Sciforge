use crate::components::powertrain::engines::turbines::assemblies::fan::{high_bypass, turbofan::Turbofan};
use super::sn;

pub fn cfm56_5b4() -> Turbofan { high_bypass::standard(sn::cfm56_5b4::D.thrust_kn, sn::cfm56_5b4::D.bypass_ratio, sn::cfm56_5b4::D.fan_diameter_m) }
pub fn cfm56_5b7() -> Turbofan { high_bypass::standard(sn::cfm56_5b7::D.thrust_kn, sn::cfm56_5b7::D.bypass_ratio, sn::cfm56_5b7::D.fan_diameter_m) }
pub fn cfm56_7b24() -> Turbofan { high_bypass::standard(sn::cfm56_7b24::D.thrust_kn, sn::cfm56_7b24::D.bypass_ratio, sn::cfm56_7b24::D.fan_diameter_m) }
pub fn cfm56_7b27() -> Turbofan { high_bypass::standard(sn::cfm56_7b27::D.thrust_kn, sn::cfm56_7b27::D.bypass_ratio, sn::cfm56_7b27::D.fan_diameter_m) }
pub fn v2527_a5() -> Turbofan { high_bypass::standard(sn::v2527_a5::D.thrust_kn, sn::v2527_a5::D.bypass_ratio, sn::v2527_a5::D.fan_diameter_m) }
pub fn v2533_a5() -> Turbofan { high_bypass::standard(sn::v2533_a5::D.thrust_kn, sn::v2533_a5::D.bypass_ratio, sn::v2533_a5::D.fan_diameter_m) }
pub fn cf6_80c2() -> Turbofan { high_bypass::standard(sn::cf6_80c2::D.thrust_kn, sn::cf6_80c2::D.bypass_ratio, sn::cf6_80c2::D.fan_diameter_m) }
pub fn cf6_80e1() -> Turbofan { high_bypass::standard(sn::cf6_80e1::D.thrust_kn, sn::cf6_80e1::D.bypass_ratio, sn::cf6_80e1::D.fan_diameter_m) }

pub fn all() -> Vec<Turbofan> {
    vec![cfm56_5b4(), cfm56_5b7(), cfm56_7b24(), cfm56_7b27(), v2527_a5(), v2533_a5(), cf6_80c2(), cf6_80e1()]
}
