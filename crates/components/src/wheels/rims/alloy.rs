#[derive(Debug, Clone)]
pub struct AlloyRim {
    pub diameter_inch: u8,
    pub width_inch: f64,
    pub bolt_pattern: String,
    pub center_bore_mm: f64,
    pub weight_kg: f64,
    pub forged: bool,
}

impl AlloyRim {
    pub fn cast(diameter_inch: u8, width_inch: f64, bolt_pattern: impl Into<String>, center_bore_mm: f64, weight_kg: f64) -> Self {
        Self { diameter_inch, width_inch, bolt_pattern: bolt_pattern.into(), center_bore_mm, weight_kg, forged: false }
    }

    pub fn forged(diameter_inch: u8, width_inch: f64, bolt_pattern: impl Into<String>, center_bore_mm: f64, weight_kg: f64) -> Self {
        Self { diameter_inch, width_inch, bolt_pattern: bolt_pattern.into(), center_bore_mm, weight_kg, forged: true }
    }
}
