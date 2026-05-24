use crate::components::powertrain::engines::thermals::assemblies::radials::r7;
use super::sn;
use super::super::PistonEngine;

pub fn gnome_7_omega() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::gnome_7_omega::D.displacement.cc,
        bore_mm: sn::gnome_7_omega::D.displacement.bore_mm,
        stroke_mm: sn::gnome_7_omega::D.displacement.stroke_mm,
        power_kw: sn::gnome_7_omega::D.power_kw,
        rpm_max: sn::gnome_7_omega::D.rpm_max,
        compression_ratio: r7::intakes::atmo::compression_ratio(),
        cylinders: r7::CYLINDERS,
        turbocharged: false,
    }
}

pub fn gnome_7_lambda() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::gnome_7_lambda::D.displacement.cc,
        bore_mm: sn::gnome_7_lambda::D.displacement.bore_mm,
        stroke_mm: sn::gnome_7_lambda::D.displacement.stroke_mm,
        power_kw: sn::gnome_7_lambda::D.power_kw,
        rpm_max: sn::gnome_7_lambda::D.rpm_max,
        compression_ratio: r7::intakes::atmo::compression_ratio(),
        cylinders: r7::CYLINDERS,
        turbocharged: false,
    }
}

pub fn all() -> Vec<PistonEngine> {
    vec![gnome_7_omega(), gnome_7_lambda()]
}
