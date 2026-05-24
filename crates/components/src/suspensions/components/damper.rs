#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamperType {
    Monotube,
    TwinTube,
    Coilover,
    Pneumatic,
}

#[derive(Debug, Clone)]
pub struct Damper {
    pub damper_type: DamperType,
    pub bore_mm: f64,
    pub stroke_mm: f64,
    pub rebound_adjustable: bool,
    pub compression_adjustable: bool,
}

impl Damper {
    pub fn twin_tube(bore_mm: f64, stroke_mm: f64) -> Self {
        Self { damper_type: DamperType::TwinTube, bore_mm, stroke_mm, rebound_adjustable: false, compression_adjustable: false }
    }

    pub fn monotube(bore_mm: f64, stroke_mm: f64) -> Self {
        Self { damper_type: DamperType::Monotube, bore_mm, stroke_mm, rebound_adjustable: true, compression_adjustable: false }
    }

    pub fn coilover(bore_mm: f64, stroke_mm: f64) -> Self {
        Self { damper_type: DamperType::Coilover, bore_mm, stroke_mm, rebound_adjustable: true, compression_adjustable: true }
    }
}
