use sciforge_core::materials::Material;
use sciforge_core::materials::alus::billet::AL_7075_T6;
use sciforge_core::materials::composites::cfrp::CFRP_T700;
use sciforge_core::materials::irons::steels::STEEL_4340;

#[allow(non_upper_case_globals)]
pub struct FlywheelMaterial;

#[allow(non_upper_case_globals)]
impl FlywheelMaterial {
    pub const Steel: &'static std::sync::LazyLock<Material> = &STEEL_4340;
    pub const BilletAluminum: &'static std::sync::LazyLock<Material> = &AL_7075_T6;
    pub const CarbonFibre: &'static std::sync::LazyLock<Material> = &CFRP_T700;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlywheelType {
    SingleMass,
    DualMass,
    Lightweight,
}

#[derive(Debug, Clone)]
pub struct Flywheel {
    pub flywheel_type: FlywheelType,
    pub material: &'static std::sync::LazyLock<Material>,
    pub mass_kg: f64,
    pub diameter_mm: f64,
    pub inertia_kgm2: f64,
}

impl Flywheel {
    pub fn single_mass(mass_kg: f64, diameter_mm: f64) -> Self {
        let inertia_kgm2 = 0.5 * mass_kg * (diameter_mm / 2000.0).powi(2);
        Self { flywheel_type: FlywheelType::SingleMass, material: FlywheelMaterial::Steel, mass_kg, diameter_mm, inertia_kgm2 }
    }

    pub fn dual_mass(mass_kg: f64, diameter_mm: f64) -> Self {
        let inertia_kgm2 = 0.5 * mass_kg * (diameter_mm / 2000.0).powi(2);
        Self { flywheel_type: FlywheelType::DualMass, material: FlywheelMaterial::Steel, mass_kg, diameter_mm, inertia_kgm2 }
    }

    pub fn lightweight(mass_kg: f64, diameter_mm: f64) -> Self {
        let inertia_kgm2 = 0.5 * mass_kg * (diameter_mm / 2000.0).powi(2);
        Self { flywheel_type: FlywheelType::Lightweight, material: FlywheelMaterial::BilletAluminum, mass_kg, diameter_mm, inertia_kgm2 }
    }
}
