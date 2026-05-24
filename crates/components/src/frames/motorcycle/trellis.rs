#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrellisMaterial {
    Steel,
    ChromeMoly,
    Aluminum,
}

#[derive(Debug, Clone)]
pub struct TrellisFrame {
    pub material: TrellisMaterial,
    pub tube_diameter_mm: f64,
    pub tube_wall_thickness_mm: f64,
    pub weight_kg: f64,
}

impl TrellisFrame {
    pub fn steel(tube_diameter_mm: f64, tube_wall_thickness_mm: f64, weight_kg: f64) -> Self {
        Self { material: TrellisMaterial::Steel, tube_diameter_mm, tube_wall_thickness_mm, weight_kg }
    }

    pub fn chrome_moly(tube_diameter_mm: f64, tube_wall_thickness_mm: f64, weight_kg: f64) -> Self {
        Self { material: TrellisMaterial::ChromeMoly, tube_diameter_mm, tube_wall_thickness_mm, weight_kg }
    }
}
