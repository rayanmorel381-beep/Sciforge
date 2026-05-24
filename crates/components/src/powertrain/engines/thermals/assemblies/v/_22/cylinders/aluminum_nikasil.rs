use crate::powertrain::engines::thermals::parts::cylinders::CylinderBank;

use super::super::{CYLINDERS, Displacement};

pub fn standard(d: &Displacement) -> CylinderBank {
    CylinderBank {
        nikasil_lined: true,
        ..CylinderBank::v(CYLINDERS, d.bore_mm, d.bank_angle_deg)
    }
}
