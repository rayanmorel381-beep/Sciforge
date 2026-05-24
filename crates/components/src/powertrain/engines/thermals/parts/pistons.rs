use sciforge_core::materials::Material;
use sciforge_core::materials::alus::cast::AC4B;
use sciforge_core::materials::alus::forged::AL_2618;
use sciforge_core::materials::irons::steels::STEEL_4340;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[allow(non_upper_case_globals)]
pub struct PistonMaterial;

#[allow(non_upper_case_globals)]
impl PistonMaterial {
    pub const CastAluminium: &'static std::sync::LazyLock<Material> = &AC4B;
    pub const ForgedAluminium: &'static std::sync::LazyLock<Material> = &AL_2618;
    pub const ForgedSteel: &'static std::sync::LazyLock<Material> = &STEEL_4340;
    pub const Titanium: &'static std::sync::LazyLock<Material> = &TI6AL4V_GR5;
}

#[derive(Debug, Clone)]
pub struct PistonSet {
    pub piston_count: u8,
    pub diameter_mm: f64,
    pub compression_height_mm: f64,
    pub material: &'static std::sync::LazyLock<Material>,
    pub coated: bool,
}

impl PistonSet {
    pub fn cast(count: u8, diameter_mm: f64, stroke_mm: f64) -> Self {
        Self {
            piston_count: count,
            diameter_mm,
            compression_height_mm: stroke_mm * 0.35,
            material: PistonMaterial::CastAluminium,
            coated: false,
        }
    }

    pub fn forged(count: u8, diameter_mm: f64, stroke_mm: f64) -> Self {
        Self {
            piston_count: count,
            diameter_mm,
            compression_height_mm: stroke_mm * 0.32,
            material: PistonMaterial::ForgedAluminium,
            coated: true,
        }
    }

    pub fn high_performance(count: u8, diameter_mm: f64, stroke_mm: f64) -> Self {
        Self {
            piston_count: count,
            diameter_mm,
            compression_height_mm: stroke_mm * 0.30,
            material: PistonMaterial::Titanium,
            coated: true,
        }
    }
}
