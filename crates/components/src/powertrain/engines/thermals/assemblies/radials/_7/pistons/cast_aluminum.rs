use crate::powertrain::engines::thermals::parts::pistons::PistonSet;

use super::super::{CYLINDERS, Displacement};

pub fn standard(d: &Displacement) -> PistonSet {
    PistonSet::cast(CYLINDERS, d.bore_mm, d.stroke_mm)
}
