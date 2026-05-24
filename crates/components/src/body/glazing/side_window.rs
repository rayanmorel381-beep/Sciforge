use super::windshield::GlazingType;

#[derive(Debug, Clone)]
pub struct SideWindow {
    pub glazing_type: GlazingType,
    pub thickness_mm: f64,
    pub tinted: bool,
    pub frameless: bool,
    pub electric: bool,
}

impl SideWindow {
    pub fn standard(thickness_mm: f64) -> Self {
        Self { glazing_type: GlazingType::Tempered, thickness_mm, tinted: false, frameless: false, electric: true }
    }

    pub fn tinted(thickness_mm: f64) -> Self {
        Self { glazing_type: GlazingType::Tempered, thickness_mm, tinted: true, frameless: false, electric: true }
    }

    pub fn frameless(thickness_mm: f64) -> Self {
        Self { glazing_type: GlazingType::Tempered, thickness_mm, tinted: false, frameless: true, electric: true }
    }
}
