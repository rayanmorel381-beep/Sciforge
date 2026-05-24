use crate::components::powertrain::engines::thermals::assemblies::flat::f4;
use super::sn;
use super::super::super::PistonEngine;

pub fn rotax_914() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::rotax_914::D.displacement.cc,
        bore_mm: sn::rotax_914::D.displacement.bore_mm,
        stroke_mm: sn::rotax_914::D.displacement.stroke_mm,
        power_kw: sn::rotax_914::D.power_kw,
        rpm_max: sn::rotax_914::D.rpm_max,
        compression_ratio: f4::intakes::turbo::compression_ratio_standard(),
        cylinders: f4::CYLINDERS,
        turbocharged: true,
    }
}

pub fn all() -> Vec<PistonEngine> {
    vec![rotax_914()]
}
