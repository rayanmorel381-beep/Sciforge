use crate::powertrain::engines::thermals::parts::crankshafts::{
    CrankPin, Crankshaft,
};

use super::super::{CYLINDERS, Displacement};

pub const PIN_SPACING_DEG: f64 = 90.0;

pub fn standard(d: &Displacement) -> Crankshaft {
    Crankshaft {
        pin_layout: CrankPin::Cross,
        throw_count: CYLINDERS / 2,
        stroke_mm: d.stroke_mm,
        main_bearing_count: 11,
        forged: false,
    }
}

pub fn forged(d: &Displacement) -> Crankshaft {
    Crankshaft {
        pin_layout: CrankPin::Cross,
        throw_count: CYLINDERS / 2,
        stroke_mm: d.stroke_mm,
        main_bearing_count: 11,
        forged: true,
    }
}

pub fn split_pin(d: &Displacement) -> Crankshaft {
    Crankshaft {
        pin_layout: CrankPin::Cross,
        throw_count: CYLINDERS,
        stroke_mm: d.stroke_mm,
        main_bearing_count: 11,
        forged: true,
    }
}
