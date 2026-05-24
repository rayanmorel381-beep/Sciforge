use crate::components::powertrain::engines::turbines::assemblies::propeller::{reduction_gear, turboprop::Turboprop};
use super::sn;

pub fn pt6a_27() -> Turboprop { reduction_gear::standard(sn::pt6a_27::D.shaft_power_kw, sn::pt6a_27::D.propeller_diameter_m, 15.0) }
pub fn pt6a_60a() -> Turboprop { reduction_gear::standard(sn::pt6a_60a::D.shaft_power_kw, sn::pt6a_60a::D.propeller_diameter_m, 15.0) }
pub fn tpe331_10() -> Turboprop { reduction_gear::standard(sn::tpe331_10::D.shaft_power_kw, sn::tpe331_10::D.propeller_diameter_m, 12.5) }
pub fn tpe331_12ub() -> Turboprop { reduction_gear::standard(sn::tpe331_12ub::D.shaft_power_kw, sn::tpe331_12ub::D.propeller_diameter_m, 12.5) }
pub fn ct7_9b() -> Turboprop { reduction_gear::standard(sn::ct7_9b::D.shaft_power_kw, sn::ct7_9b::D.propeller_diameter_m, 15.5) }
pub fn tvd_10b() -> Turboprop { reduction_gear::standard(sn::tvd_10b::D.shaft_power_kw, sn::tvd_10b::D.propeller_diameter_m, 12.0) }

pub fn all() -> Vec<Turboprop> {
    vec![pt6a_27(), pt6a_60a(), tpe331_10(), tpe331_12ub(), ct7_9b(), tvd_10b()]
}
