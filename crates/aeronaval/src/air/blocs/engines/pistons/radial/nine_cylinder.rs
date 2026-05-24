use crate::components::powertrain::engines::thermals::assemblies::radials::r9;
use super::sn;
use super::super::PistonEngine;

pub fn wright_j5() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::wright_j5::D.displacement.cc,
        bore_mm: sn::wright_j5::D.displacement.bore_mm,
        stroke_mm: sn::wright_j5::D.displacement.stroke_mm,
        power_kw: sn::wright_j5::D.power_kw,
        rpm_max: sn::wright_j5::D.rpm_max,
        compression_ratio: r9::intakes::atmo::compression_ratio(),
        cylinders: r9::CYLINDERS,
        turbocharged: false,
    }
}

pub fn pw_r985_wasp_jr() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::pw_r985_wasp_jr::D.displacement.cc,
        bore_mm: sn::pw_r985_wasp_jr::D.displacement.bore_mm,
        stroke_mm: sn::pw_r985_wasp_jr::D.displacement.stroke_mm,
        power_kw: sn::pw_r985_wasp_jr::D.power_kw,
        rpm_max: sn::pw_r985_wasp_jr::D.rpm_max,
        compression_ratio: r9::intakes::atmo::compression_ratio(),
        cylinders: r9::CYLINDERS,
        turbocharged: false,
    }
}

pub fn pw_r1340_wasp() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::pw_r1340_wasp::D.displacement.cc,
        bore_mm: sn::pw_r1340_wasp::D.displacement.bore_mm,
        stroke_mm: sn::pw_r1340_wasp::D.displacement.stroke_mm,
        power_kw: sn::pw_r1340_wasp::D.power_kw,
        rpm_max: sn::pw_r1340_wasp::D.rpm_max,
        compression_ratio: r9::intakes::atmo::compression_ratio(),
        cylinders: r9::CYLINDERS,
        turbocharged: false,
    }
}

pub fn wright_r1820() -> PistonEngine {
    PistonEngine {
        displacement_cc: sn::wright_r1820::D.displacement.cc,
        bore_mm: sn::wright_r1820::D.displacement.bore_mm,
        stroke_mm: sn::wright_r1820::D.displacement.stroke_mm,
        power_kw: sn::wright_r1820::D.power_kw,
        rpm_max: sn::wright_r1820::D.rpm_max,
        compression_ratio: r9::intakes::atmo::compression_ratio(),
        cylinders: r9::CYLINDERS,
        turbocharged: false,
    }
}

pub fn all() -> Vec<PistonEngine> {
    vec![wright_j5(), pw_r985_wasp_jr(), pw_r1340_wasp(), wright_r1820()]
}
