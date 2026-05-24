#[derive(Debug, Clone)]
pub struct BackboneFrame {
    pub tunnel_diameter_mm: f64,
    pub tunnel_length_mm: f64,
    pub torsional_rigidity_nm_per_deg: f64,
    pub weight_kg: f64,
}

impl BackboneFrame {
    pub fn new(tunnel_diameter_mm: f64, tunnel_length_mm: f64, torsional_rigidity_nm_per_deg: f64, weight_kg: f64) -> Self {
        Self { tunnel_diameter_mm, tunnel_length_mm, torsional_rigidity_nm_per_deg, weight_kg }
    }
}
