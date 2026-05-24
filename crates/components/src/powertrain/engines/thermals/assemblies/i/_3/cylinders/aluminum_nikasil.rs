use crate::powertrain::engines::thermals::parts::cylinders::CylinderBank;

use super::super::{CYLINDERS, Displacement};

pub fn standard(d: &Displacement) -> CylinderBank {
    CylinderBank {
        nikasil_lined: true,
        ..CylinderBank::inline(CYLINDERS, d.bore_mm, 0.0)
    }
}
