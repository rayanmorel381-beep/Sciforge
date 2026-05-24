#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LadderMaterial {
    Steel,
    BoxSection,
    Hydroformed,
}

#[derive(Debug, Clone)]
pub struct LadderFrame {
    pub material: LadderMaterial,
    pub rail_width_mm: f64,
    pub crossmember_count: u8,
    pub torsional_rigidity_nm_per_deg: f64,
    pub weight_kg: f64,
}

impl LadderFrame {
    pub fn steel(rail_width_mm: f64, crossmember_count: u8, weight_kg: f64) -> Self {
        Self { material: LadderMaterial::Steel, rail_width_mm, crossmember_count, torsional_rigidity_nm_per_deg: 5000.0, weight_kg }
    }

    pub fn hydroformed(rail_width_mm: f64, crossmember_count: u8, weight_kg: f64) -> Self {
        Self {
            material: LadderMaterial::Hydroformed,
            rail_width_mm,
            crossmember_count,
            torsional_rigidity_nm_per_deg: 9000.0,
            weight_kg,
        }
    }
}
