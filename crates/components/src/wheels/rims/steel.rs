#[derive(Debug, Clone)]
pub struct SteelRim {
    pub diameter_inch: u8,
    pub width_inch: f64,
    pub bolt_pattern: String,
    pub center_bore_mm: f64,
    pub weight_kg: f64,
    pub stamped: bool,
}

impl SteelRim {
    pub fn stamped(diameter_inch: u8, width_inch: f64, bolt_pattern: impl Into<String>, center_bore_mm: f64, weight_kg: f64) -> Self {
        Self { diameter_inch, width_inch, bolt_pattern: bolt_pattern.into(), center_bore_mm, weight_kg, stamped: true }
    }

    pub fn rolled(diameter_inch: u8, width_inch: f64, bolt_pattern: impl Into<String>, center_bore_mm: f64, weight_kg: f64) -> Self {
        Self { diameter_inch, width_inch, bolt_pattern: bolt_pattern.into(), center_bore_mm, weight_kg, stamped: false }
    }
}
