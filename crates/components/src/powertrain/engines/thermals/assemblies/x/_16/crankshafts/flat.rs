use crate::powertrain::engines::thermals::parts::crankshafts::{
    CrankPin, Crankshaft,
};

use super::super::{CYLINDERS, Displacement};

pub const PIN_SPACING_DEG: f64 = 180.0;

pub fn standard(d: &Displacement) -> Crankshaft {
    Crankshaft {
        pin_layout: CrankPin::Flat,
        throw_count: CYLINDERS / 2,
        stroke_mm: d.stroke_mm,
        main_bearing_count: 9,
        forged: true,
    }
}

pub fn race(d: &Displacement) -> Crankshaft {
    standard(d)
}
