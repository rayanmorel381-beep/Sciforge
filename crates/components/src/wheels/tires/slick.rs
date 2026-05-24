#[derive(Debug, Clone)]
pub struct SlickTire {
    pub width_mm: u16,
    pub rim_diameter_inch: u8,
    pub compound: SlickCompound,
    pub operating_temp_min_c: i16,
    pub operating_temp_max_c: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlickCompound {
    SuperSoft,
    Soft,
    Medium,
    Hard,
    Intermediate,
}

impl SlickTire {
    pub fn new(width_mm: u16, rim_diameter_inch: u8, compound: SlickCompound) -> Self {
        let (min, max) = match compound {
            SlickCompound::SuperSoft => (70, 100),
            SlickCompound::Soft => (80, 110),
            SlickCompound::Medium => (90, 120),
            SlickCompound::Hard => (100, 130),
            SlickCompound::Intermediate => (40, 80),
        };
        Self { width_mm, rim_diameter_inch, compound, operating_temp_min_c: min, operating_temp_max_c: max }
    }
}
