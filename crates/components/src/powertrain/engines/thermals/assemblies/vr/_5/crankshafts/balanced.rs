use crate::powertrain::engines::thermals::parts::crankshafts::{CrankPin, Crankshaft};
use super::super::{CYLINDERS, Displacement};

pub const PIN_SPACING_DEG: f64 = 144.0;

pub fn standard(d: &Displacement) -> Crankshaft {
    Crankshaft {
        pin_layout: CrankPin::Irregular,
        throw_count: CYLINDERS,
        stroke_mm: d.stroke_mm,
        main_bearing_count: 6,
        forged: true,
    }
}

pub fn forged(d: &Displacement) -> Crankshaft { standard(d) }
pub fn split_pin(d: &Displacement) -> Crankshaft { standard(d) }
