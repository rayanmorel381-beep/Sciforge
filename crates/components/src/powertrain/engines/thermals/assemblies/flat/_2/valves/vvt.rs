use crate::powertrain::engines::thermals::parts::valves::{
    ValveTrain, VvtSystem,
};

use super::super::CYLINDERS;

pub fn intake_only() -> ValveTrain {
    ValveTrain::four_valve_dohc(CYLINDERS, VvtSystem::SingleVvt)
}

pub fn dual_independent() -> ValveTrain {
    ValveTrain::four_valve_dohc(CYLINDERS, VvtSystem::DualVvt)
}

pub fn continuous() -> ValveTrain {
    ValveTrain::four_valve_dohc(CYLINDERS, VvtSystem::ContinuousVvt)
}
