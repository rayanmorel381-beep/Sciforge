#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkinMaterial {
    Aluminum,
    CarbonFibre,
    GlassFibre,
    TitaniumAlloy,
}

#[derive(Debug, Clone)]
pub struct StressedSkin {
    pub skin_material: SkinMaterial,
    pub skin_thickness_mm: f64,
    pub torsional_rigidity_nm_per_deg: f64,
    pub bonded: bool,
    pub weight_kg: f64,
}

impl StressedSkin {
    pub fn aluminum(skin_thickness_mm: f64, torsional_rigidity_nm_per_deg: f64, weight_kg: f64) -> Self {
        Self { skin_material: SkinMaterial::Aluminum, skin_thickness_mm, torsional_rigidity_nm_per_deg, bonded: true, weight_kg }
    }

    pub fn carbon(skin_thickness_mm: f64, torsional_rigidity_nm_per_deg: f64, weight_kg: f64) -> Self {
        Self { skin_material: SkinMaterial::CarbonFibre, skin_thickness_mm, torsional_rigidity_nm_per_deg, bonded: true, weight_kg }
    }
}
