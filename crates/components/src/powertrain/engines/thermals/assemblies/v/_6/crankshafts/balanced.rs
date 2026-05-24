use crate::powertrain::engines::thermals::parts::crankshafts::{
    CrankPin, Crankshaft,
};

use super::super::{CYLINDERS, Displacement};

const FOUR_STROKE_CYCLE_DEG: f64 = 720.0;

pub fn pin_spacing_deg() -> f64 {
    FOUR_STROKE_CYCLE_DEG / CYLINDERS as f64
}

fn layout_for_spacing(spacing_deg: f64) -> CrankPin {
    if (spacing_deg - 180.0).abs() < 0.5 {
        CrankPin::Flat
    } else if (spacing_deg - 90.0).abs() < 0.5 {
        CrankPin::Cross
    } else if (spacing_deg - 60.0).abs() < 0.5 || (spacing_deg - 120.0).abs() < 0.5 {
        CrankPin::Plane60
    } else {
        CrankPin::Irregular
    }
}

pub fn standard(d: &Displacement) -> Crankshaft {
    let spacing = pin_spacing_deg();
    Crankshaft {
        pin_layout: layout_for_spacing(spacing),
        throw_count: CYLINDERS / 2,
        stroke_mm: d.stroke_mm,
        main_bearing_count: (CYLINDERS / 2) + 1,
        forged: false,
    }
}

pub fn forged(d: &Displacement) -> Crankshaft {
    let spacing = pin_spacing_deg();
    Crankshaft {
        pin_layout: layout_for_spacing(spacing),
        throw_count: CYLINDERS / 2,
        stroke_mm: d.stroke_mm,
        main_bearing_count: (CYLINDERS / 2) + 1,
        forged: true,
    }
}

pub fn split_pin(d: &Displacement) -> Crankshaft {
    let spacing = pin_spacing_deg();
    Crankshaft {
        pin_layout: layout_for_spacing(spacing),
        throw_count: CYLINDERS,
        stroke_mm: d.stroke_mm,
        main_bearing_count: (CYLINDERS / 2) + 1,
        forged: true,
    }
}
