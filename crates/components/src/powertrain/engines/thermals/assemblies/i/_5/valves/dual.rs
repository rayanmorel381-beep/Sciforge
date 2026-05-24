use crate::powertrain::engines::thermals::parts::valves::{
    ValveTrain, VvtSystem,
};

use super::super::CYLINDERS;

pub fn standard() -> ValveTrain {
    ValveTrain::four_valve_dohc(CYLINDERS, VvtSystem::None)
}
