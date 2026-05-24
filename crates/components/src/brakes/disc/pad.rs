#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadCompound {
    Organic,
    SemiMetallic,
    FullMetallic,
    Ceramic,
    Carbon,
}

#[derive(Debug, Clone)]
pub struct BrakePad {
    pub compound: PadCompound,
    pub thickness_mm: f64,
    pub friction_coefficient: f64,
    pub operating_temp_max_c: u16,
}

impl BrakePad {
    pub fn organic() -> Self {
        Self { compound: PadCompound::Organic, thickness_mm: 12.0, friction_coefficient: 0.35, operating_temp_max_c: 300 }
    }

    pub fn semi_metallic() -> Self {
        Self { compound: PadCompound::SemiMetallic, thickness_mm: 12.0, friction_coefficient: 0.42, operating_temp_max_c: 500 }
    }

    pub fn ceramic() -> Self {
        Self { compound: PadCompound::Ceramic, thickness_mm: 12.0, friction_coefficient: 0.45, operating_temp_max_c: 650 }
    }

    pub fn carbon() -> Self {
        Self { compound: PadCompound::Carbon, thickness_mm: 30.0, friction_coefficient: 0.55, operating_temp_max_c: 1000 }
    }
}
