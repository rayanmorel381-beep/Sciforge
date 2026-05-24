use crate::powertrain::engines::thermals::parts::valves::ValveTrain;

use super::super::CYLINDERS;

pub fn standard() -> ValveTrain {
    ValveTrain::two_valve_ohv(CYLINDERS)
}
