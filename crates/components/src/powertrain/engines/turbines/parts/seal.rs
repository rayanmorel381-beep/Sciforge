use sciforge_core::materials::nickels::inconel::INCONEL_718;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SealType {
    Labyrinth,
    BrushSeal,
    TipClearance,
    CarbonFace,
}

#[derive(Debug, Clone)]
pub struct Seal {
    pub seal_type: SealType,
    pub leakage_fraction: f64,
    pub max_temp_k: f64,
    pub max_pressure_bar: f64,
    pub material_density_kg_m3: f64,
}

impl Seal {
    pub fn labyrinth(leakage_fraction: f64) -> Self {
        Self {
            seal_type: SealType::Labyrinth,
            leakage_fraction,
            max_temp_k: 1_100.0,
            max_pressure_bar: 60.0,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
        }
    }

    pub fn brush(leakage_fraction: f64) -> Self {
        Self {
            seal_type: SealType::BrushSeal,
            leakage_fraction,
            max_temp_k: 900.0,
            max_pressure_bar: 40.0,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
        }
    }

    pub fn tip_clearance(gap_mm: f64, blade_height_mm: f64) -> Self {
        Self {
            seal_type: SealType::TipClearance,
            leakage_fraction: gap_mm / blade_height_mm,
            max_temp_k: 1_400.0,
            max_pressure_bar: 50.0,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
        }
    }

    pub fn efficiency_penalty(&self) -> f64 {
        self.leakage_fraction * 2.5
    }
}
