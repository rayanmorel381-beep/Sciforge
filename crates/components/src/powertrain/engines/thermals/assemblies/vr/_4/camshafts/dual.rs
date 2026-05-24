use crate::powertrain::engines::thermals::parts::camshafts::Camshaft;
use super::super::CYLINDERS;

pub fn standard() -> Camshaft { Camshaft::dohc(CYLINDERS as u16, false) }
pub fn vvt() -> Camshaft { Camshaft::dohc(CYLINDERS as u16, true) }
