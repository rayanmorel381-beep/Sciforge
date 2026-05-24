use crate::powertrain::engines::thermals::parts::crankshafts::{
    CrankPin, Crankshaft,
};

use super::super::{CYLINDERS, Displacement};

pub fn standard(d: &Displacement) -> Crankshaft {
    let mut c = Crankshaft::radial(CYLINDERS, d.stroke_mm);
    c.pin_layout = CrankPin::Flat;
    c.forged = false;
    c
}

pub fn forged(d: &Displacement) -> Crankshaft {
    let mut c = Crankshaft::radial(CYLINDERS, d.stroke_mm);
    c.pin_layout = CrankPin::Flat;
    c.forged = true;
    c
}
