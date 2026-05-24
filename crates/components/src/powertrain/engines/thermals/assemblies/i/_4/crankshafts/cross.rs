use crate::powertrain::engines::thermals::parts::crankshafts::{
    CrankPin, Crankshaft,
};

use super::super::{CYLINDERS, Displacement};

pub fn standard(d: &Displacement) -> Crankshaft {
    Crankshaft {
        pin_layout: CrankPin::Cross,
        throw_count: CYLINDERS,
        stroke_mm: d.stroke_mm,
        main_bearing_count: CYLINDERS + 1,
        forged: false,
    }
}

pub fn forged(d: &Displacement) -> Crankshaft {
    Crankshaft {
        pin_layout: CrankPin::Cross,
        throw_count: CYLINDERS,
        stroke_mm: d.stroke_mm,
        main_bearing_count: CYLINDERS + 1,
        forged: true,
    }
}
