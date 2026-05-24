use crate::components::frames::monocoque::{SkinMaterial, StressedSkin};

#[derive(Debug, Clone)]
pub struct HullKit {
    pub pressure_hull: StressedSkin,
    pub fairing: StressedSkin,
}

pub fn shallow() -> HullKit {
    HullKit {
        pressure_hull: StressedSkin {
            skin_material: SkinMaterial::TitaniumAlloy,
            skin_thickness_mm: 8.0,
            torsional_rigidity_nm_per_deg: 42_000.0,
            bonded: true,
            weight_kg: 18.0,
        },
        fairing: StressedSkin {
            skin_material: SkinMaterial::TitaniumAlloy,
            skin_thickness_mm: 3.0,
            torsional_rigidity_nm_per_deg: 9_000.0,
            bonded: true,
            weight_kg: 4.0,
        },
    }
}

pub fn deep() -> HullKit {
    HullKit {
        pressure_hull: StressedSkin {
            skin_material: SkinMaterial::TitaniumAlloy,
            skin_thickness_mm: 18.0,
            torsional_rigidity_nm_per_deg: 110_000.0,
            bonded: true,
            weight_kg: 48.0,
        },
        fairing: StressedSkin {
            skin_material: SkinMaterial::TitaniumAlloy,
            skin_thickness_mm: 6.0,
            torsional_rigidity_nm_per_deg: 24_000.0,
            bonded: true,
            weight_kg: 10.0,
        },
    }
}

pub fn all() -> Vec<HullKit> {
    vec![shallow(), deep()]
}
