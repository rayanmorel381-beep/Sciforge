use crate::powertrain::engines::thermals::parts::pistons::{
    PistonMaterial, PistonSet,
};

use super::super::{CYLINDERS, Displacement};

pub fn standard(d: &Displacement) -> PistonSet {
    PistonSet {
        piston_count: CYLINDERS,
        diameter_mm: d.bore_mm,
        compression_height_mm: d.stroke_mm * 0.31,
        material: PistonMaterial::ForgedSteel,
        coated: true,
    }
}
