use super::windshield::GlazingType;

#[derive(Debug, Clone)]
pub struct RearWindow {
    pub glazing_type: GlazingType,
    pub thickness_mm: f64,
    pub tinted: bool,
    pub defroster: bool,
    pub wiper: bool,
}

impl RearWindow {
    pub fn standard(thickness_mm: f64) -> Self {
        Self { glazing_type: GlazingType::Tempered, thickness_mm, tinted: false, defroster: true, wiper: false }
    }

    pub fn with_wiper(thickness_mm: f64) -> Self {
        Self { glazing_type: GlazingType::Tempered, thickness_mm, tinted: false, defroster: true, wiper: true }
    }
}
