use crate::components::powertrain::engines::turbines::assemblies::fan::{low_bypass, turbofan::Turbofan};
use super::sn;

pub fn rd_93() -> Turbofan { low_bypass::military(sn::rd_93::D.thrust_kn, sn::rd_93::D.bypass_ratio) }
pub fn ej200() -> Turbofan { low_bypass::military(sn::ej200::D.thrust_kn, sn::ej200::D.bypass_ratio) }

pub fn all() -> Vec<Turbofan> {
    vec![rd_93(), ej200()]
}
