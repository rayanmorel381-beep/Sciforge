use crate::components::powertrain::engines::thermals::assemblies::flat::f4;
use super::sn;
use super::super::super::PistonEngine;

pub fn continental_o200_a() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::continental_o200_a::D.displacement.cc,
        bore_mm: sn::continental_o200_a::D.displacement.bore_mm,
        stroke_mm: sn::continental_o200_a::D.displacement.stroke_mm,
        power_kw: sn::continental_o200_a::D.power_kw,
        rpm_max: sn::continental_o200_a::D.rpm_max,
        compression_ratio: f4::intakes::atmo::compression_ratio(),
        cylinders: f4::CYLINDERS,
        turbocharged: false,
    }
}

pub fn lycoming_o320_b() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::lycoming_o320_b::D.displacement.cc,
        bore_mm: sn::lycoming_o320_b::D.displacement.bore_mm,
        stroke_mm: sn::lycoming_o320_b::D.displacement.stroke_mm,
        power_kw: sn::lycoming_o320_b::D.power_kw,
        rpm_max: sn::lycoming_o320_b::D.rpm_max,
        compression_ratio: f4::intakes::atmo::compression_ratio(),
        cylinders: f4::CYLINDERS,
        turbocharged: false,
    }
}

pub fn lycoming_o360_a1a() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::lycoming_o360_a1a::D.displacement.cc,
        bore_mm: sn::lycoming_o360_a1a::D.displacement.bore_mm,
        stroke_mm: sn::lycoming_o360_a1a::D.displacement.stroke_mm,
        power_kw: sn::lycoming_o360_a1a::D.power_kw,
        rpm_max: sn::lycoming_o360_a1a::D.rpm_max,
        compression_ratio: f4::intakes::atmo::compression_ratio(),
        cylinders: f4::CYLINDERS,
        turbocharged: false,
    }
}

pub fn rotax_912_uls() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::rotax_912_uls::D.displacement.cc,
        bore_mm: sn::rotax_912_uls::D.displacement.bore_mm,
        stroke_mm: sn::rotax_912_uls::D.displacement.stroke_mm,
        power_kw: sn::rotax_912_uls::D.power_kw,
        rpm_max: sn::rotax_912_uls::D.rpm_max,
        compression_ratio: f4::intakes::atmo::compression_ratio(),
        cylinders: f4::CYLINDERS,
        turbocharged: false,
    }
}

pub fn all() -> Vec<PistonEngine> {
    vec![continental_o200_a(), lycoming_o320_b(), lycoming_o360_a1a(), rotax_912_uls()]
}
