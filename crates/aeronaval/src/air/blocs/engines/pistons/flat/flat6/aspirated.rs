use crate::components::powertrain::engines::thermals::assemblies::flat::f6;
use super::sn;
use super::super::super::PistonEngine;

pub fn lycoming_io540_k() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::lycoming_io540_k::D.displacement.cc,
        bore_mm: sn::lycoming_io540_k::D.displacement.bore_mm,
        stroke_mm: sn::lycoming_io540_k::D.displacement.stroke_mm,
        power_kw: sn::lycoming_io540_k::D.power_kw,
        rpm_max: sn::lycoming_io540_k::D.rpm_max,
        compression_ratio: f6::intakes::atmo::compression_ratio(),
        cylinders: f6::CYLINDERS,
        turbocharged: false,
    }
}

pub fn continental_io520_ba() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::continental_io520_ba::D.displacement.cc,
        bore_mm: sn::continental_io520_ba::D.displacement.bore_mm,
        stroke_mm: sn::continental_io520_ba::D.displacement.stroke_mm,
        power_kw: sn::continental_io520_ba::D.power_kw,
        rpm_max: sn::continental_io520_ba::D.rpm_max,
        compression_ratio: f6::intakes::atmo::compression_ratio(),
        cylinders: f6::CYLINDERS,
        turbocharged: false,
    }
}

pub fn continental_io550_n() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::continental_io550_n::D.displacement.cc,
        bore_mm: sn::continental_io550_n::D.displacement.bore_mm,
        stroke_mm: sn::continental_io550_n::D.displacement.stroke_mm,
        power_kw: sn::continental_io550_n::D.power_kw,
        rpm_max: sn::continental_io550_n::D.rpm_max,
        compression_ratio: f6::intakes::atmo::compression_ratio(),
        cylinders: f6::CYLINDERS,
        turbocharged: false,
    }
}

pub fn all() -> Vec<PistonEngine> {
    vec![lycoming_io540_k(), continental_io520_ba(), continental_io550_n()]
}
