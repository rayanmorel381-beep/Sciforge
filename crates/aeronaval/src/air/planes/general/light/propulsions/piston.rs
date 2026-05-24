use crate::components::powertrain::engines::thermals::assemblies::flat::{f4, f6};

#[derive(Debug, Clone)]
pub enum PistonEngine {
    FlatFour { displacement: f4::Displacement, power_kw: f64, torque_nm: f64, compression_ratio: f64 },
    FlatSix  { displacement: f6::Displacement, power_kw: f64, torque_nm: f64, compression_ratio: f64 },
}

pub fn flat_four() -> PistonEngine {
    PistonEngine::FlatFour {
        displacement: f4::Displacement { cc: 5_916, bore_mm: 130.2, stroke_mm: 111.1 },
        power_kw: 134.0,
        torque_nm: 214.0,
        compression_ratio: f4::intakes::atmo::compression_ratio(),
    }
}

pub fn flat_four_turbo() -> PistonEngine {
    PistonEngine::FlatFour {
        displacement: f4::Displacement { cc: 5_916, bore_mm: 130.2, stroke_mm: 111.1 },
        power_kw: 160.0,
        torque_nm: 280.0,
        compression_ratio: f4::intakes::turbo::compression_ratio_standard(),
    }
}

pub fn flat_six() -> PistonEngine {
    PistonEngine::FlatSix {
        displacement: f6::Displacement { cc: 8_899, bore_mm: 130.2, stroke_mm: 111.1 },
        power_kw: 224.0,
        torque_nm: 390.0,
        compression_ratio: f6::intakes::atmo::compression_ratio(),
    }
}

pub fn flat_six_turbo() -> PistonEngine {
    PistonEngine::FlatSix {
        displacement: f6::Displacement { cc: 8_523, bore_mm: 133.4, stroke_mm: 101.6 },
        power_kw: 240.0,
        torque_nm: 430.0,
        compression_ratio: f6::intakes::turbo::compression_ratio_standard(),
    }
}

pub fn all() -> Vec<PistonEngine> {
    vec![flat_four(), flat_four_turbo(), flat_six(), flat_six_turbo()]
}
