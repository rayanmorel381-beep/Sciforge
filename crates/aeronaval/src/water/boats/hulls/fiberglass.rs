use crate::components::frames::monocoque::{SkinMaterial, StressedSkin};

#[derive(Debug, Clone)]
pub struct HullKit {
    pub hull: StressedSkin,
    pub deck: StressedSkin,
}

pub fn runabout() -> HullKit {
    HullKit {
        hull: StressedSkin {
            skin_material: SkinMaterial::GlassFibre,
            skin_thickness_mm: 6.0,
            torsional_rigidity_nm_per_deg: 28_000.0,
            bonded: true,
            weight_kg: 320.0,
        },
        deck: StressedSkin {
            skin_material: SkinMaterial::GlassFibre,
            skin_thickness_mm: 4.0,
            torsional_rigidity_nm_per_deg: 12_000.0,
            bonded: true,
            weight_kg: 95.0,
        },
    }
}

pub fn cruiser() -> HullKit {
    HullKit {
        hull: StressedSkin {
            skin_material: SkinMaterial::GlassFibre,
            skin_thickness_mm: 14.0,
            torsional_rigidity_nm_per_deg: 120_000.0,
            bonded: true,
            weight_kg: 2_800.0,
        },
        deck: StressedSkin {
            skin_material: SkinMaterial::GlassFibre,
            skin_thickness_mm: 10.0,
            torsional_rigidity_nm_per_deg: 55_000.0,
            bonded: true,
            weight_kg: 780.0,
        },
    }
}

pub fn all() -> Vec<HullKit> {
    vec![runabout(), cruiser()]
}
