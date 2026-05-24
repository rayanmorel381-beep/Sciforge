#[derive(Debug, Clone)]
pub struct PerimeterFrame {
    pub beam_width_mm: f64,
    pub beam_height_mm: f64,
    pub aluminum: bool,
    pub torsional_rigidity_nm_per_deg: f64,
    pub weight_kg: f64,
}

impl PerimeterFrame {
    pub fn steel(beam_width_mm: f64, beam_height_mm: f64, weight_kg: f64) -> Self {
        Self { beam_width_mm, beam_height_mm, aluminum: false, torsional_rigidity_nm_per_deg: 8000.0, weight_kg }
    }

    pub fn aluminum(beam_width_mm: f64, beam_height_mm: f64, weight_kg: f64) -> Self {
        Self { beam_width_mm, beam_height_mm, aluminum: true, torsional_rigidity_nm_per_deg: 11000.0, weight_kg }
    }
}
