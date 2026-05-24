use crate::components::powertrain::engines::thermals::assemblies::flat::f6;
use super::sn;
use super::super::super::PistonEngine;

pub fn continental_tsio520_nb() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::continental_tsio520_nb::D.displacement.cc,
        bore_mm: sn::continental_tsio520_nb::D.displacement.bore_mm,
        stroke_mm: sn::continental_tsio520_nb::D.displacement.stroke_mm,
        power_kw: sn::continental_tsio520_nb::D.power_kw,
        rpm_max: sn::continental_tsio520_nb::D.rpm_max,
        compression_ratio: f6::intakes::turbo::compression_ratio_standard(),
        cylinders: f6::CYLINDERS,
        turbocharged: true,
    }
}

pub fn all() -> Vec<PistonEngine> {
    vec![continental_tsio520_nb()]
}
