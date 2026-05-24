#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SteelGrade {
    Mild,
    HighStrength,
    UltraHighStrength,
    Boron,
}

#[derive(Debug, Clone)]
pub struct Unibody {
    pub steel_grade: SteelGrade,
    pub aluminum_intensive: bool,
    pub torsional_rigidity_nm_per_deg: f64,
    pub crash_zones: u8,
    pub weight_kg: f64,
}

impl Unibody {
    pub fn steel(steel_grade: SteelGrade, torsional_rigidity_nm_per_deg: f64, weight_kg: f64) -> Self {
        Self { steel_grade, aluminum_intensive: false, torsional_rigidity_nm_per_deg, crash_zones: 2, weight_kg }
    }

    pub fn aluminum_intensive(torsional_rigidity_nm_per_deg: f64, weight_kg: f64) -> Self {
        Self {
            steel_grade: SteelGrade::HighStrength,
            aluminum_intensive: true,
            torsional_rigidity_nm_per_deg,
            crash_zones: 2,
            weight_kg,
        }
    }
}
