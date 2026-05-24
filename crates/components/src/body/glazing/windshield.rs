#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlazingType {
    Laminated,
    Tempered,
    Acoustic,
    Heated,
}

#[derive(Debug, Clone)]
pub struct Windshield {
    pub glazing_type: GlazingType,
    pub thickness_mm: f64,
    pub tinted: bool,
    pub uv_protected: bool,
    pub heated: bool,
}

impl Windshield {
    pub fn standard(thickness_mm: f64) -> Self {
        Self { glazing_type: GlazingType::Laminated, thickness_mm, tinted: false, uv_protected: true, heated: false }
    }

    pub fn heated(thickness_mm: f64) -> Self {
        Self { glazing_type: GlazingType::Heated, thickness_mm, tinted: false, uv_protected: true, heated: true }
    }

    pub fn acoustic(thickness_mm: f64) -> Self {
        Self { glazing_type: GlazingType::Acoustic, thickness_mm, tinted: false, uv_protected: true, heated: false }
    }
}
