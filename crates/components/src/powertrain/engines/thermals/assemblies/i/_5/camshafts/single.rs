use crate::powertrain::engines::thermals::parts::camshafts::Camshaft;

use super::super::CYLINDERS;

pub fn standard() -> Camshaft {
    Camshaft::sohc(CYLINDERS as u16, false)
}

pub fn vvt() -> Camshaft {
    Camshaft::sohc(CYLINDERS as u16, true)
}
