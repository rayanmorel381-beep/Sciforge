use crate::components::powertrain::engines::turbines::assemblies::fan::{geared, turbofan::Turbofan};
use super::sn;

pub fn pw1500g() -> Turbofan { geared::regional(sn::pw1500g::D.thrust_kn, sn::pw1500g::D.bypass_ratio, sn::pw1500g::D.fan_diameter_m) }

pub fn all() -> Vec<Turbofan> {
    vec![pw1500g()]
}
