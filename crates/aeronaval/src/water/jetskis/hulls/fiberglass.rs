use crate::components::frames::monocoque::{SkinMaterial, StressedSkin};

#[derive(Debug, Clone)]
pub struct HullKit {
    pub hull: StressedSkin,
    pub deck: StressedSkin,
}

pub fn stand_up() -> HullKit {
    HullKit {
        hull: StressedSkin {
            skin_material: SkinMaterial::GlassFibre,
            skin_thickness_mm: 4.0,
            torsional_rigidity_nm_per_deg: 9_500.0,
            bonded: true,
            weight_kg: 52.0,
        },
        deck: StressedSkin {
            skin_material: SkinMaterial::GlassFibre,
            skin_thickness_mm: 3.0,
            torsional_rigidity_nm_per_deg: 4_200.0,
            bonded: true,
            weight_kg: 18.0,
        },
    }
}

pub fn sit_down() -> HullKit {
    HullKit {
        hull: StressedSkin {
            skin_material: SkinMaterial::GlassFibre,
            skin_thickness_mm: 5.5,
            torsional_rigidity_nm_per_deg: 18_000.0,
            bonded: true,
            weight_kg: 95.0,
        },
        deck: StressedSkin {
            skin_material: SkinMaterial::GlassFibre,
            skin_thickness_mm: 4.0,
            torsional_rigidity_nm_per_deg: 8_000.0,
            bonded: true,
            weight_kg: 32.0,
        },
    }
}

pub fn all() -> Vec<HullKit> {
    vec![stand_up(), sit_down()]
}
