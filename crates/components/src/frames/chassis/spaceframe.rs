#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TubeMaterial {
    Steel,
    ChromeMoly,
    Aluminum,
    Titanium,
}

#[derive(Debug, Clone)]
pub struct Spaceframe {
    pub tube_material: TubeMaterial,
    pub tube_diameter_mm: f64,
    pub tube_wall_thickness_mm: f64,
    pub torsional_rigidity_nm_per_deg: f64,
    pub weight_kg: f64,
}

impl Spaceframe {
    pub fn steel(tube_diameter_mm: f64, tube_wall_thickness_mm: f64, weight_kg: f64) -> Self {
        Self { tube_material: TubeMaterial::Steel, tube_diameter_mm, tube_wall_thickness_mm, torsional_rigidity_nm_per_deg: 12000.0, weight_kg }
    }

    pub fn chrome_moly(tube_diameter_mm: f64, tube_wall_thickness_mm: f64, weight_kg: f64) -> Self {
        Self {
            tube_material: TubeMaterial::ChromeMoly,
            tube_diameter_mm,
            tube_wall_thickness_mm,
            torsional_rigidity_nm_per_deg: 18000.0,
            weight_kg,
        }
    }

    pub fn aluminum(tube_diameter_mm: f64, tube_wall_thickness_mm: f64, weight_kg: f64) -> Self {
        Self { tube_material: TubeMaterial::Aluminum, tube_diameter_mm, tube_wall_thickness_mm, torsional_rigidity_nm_per_deg: 14000.0, weight_kg }
    }
}
