use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static AZ91: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Mg"),
    name: "AZ91 cast magnesium",
    formula: "Mg",
    family: MaterialFamily::Magnesium,
    density_kg_m3: 1810.0,
    young_modulus_pa: 45.0e9,
    poisson_ratio: 0.35,
    yield_strength_pa: 160.0e6,
    ultimate_strength_pa: 240.0e6,
    thermal_conductivity_w_mk: 51.0,
    thermal_expansion_per_k: 26.0e-6,
    specific_heat_j_kgk: 1050.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 280.0e6,
    fatigue_strength_exponent: -0.11,
    hardness_hv: 75.0,
    cost_eur_kg: 4.5,
});

pub static AZ31: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Mg"),
    name: "AZ31 wrought magnesium",
    formula: "Mg",
    family: MaterialFamily::Magnesium,
    density_kg_m3: 1770.0,
    young_modulus_pa: 45.0e9,
    poisson_ratio: 0.35,
    yield_strength_pa: 200.0e6,
    ultimate_strength_pa: 290.0e6,
    thermal_conductivity_w_mk: 96.0,
    thermal_expansion_per_k: 26.0e-6,
    specific_heat_j_kgk: 1040.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 320.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 73.0,
    cost_eur_kg: 5.0,
});

pub static ZK60: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Mg"),
    name: "ZK60 high-strength magnesium",
    formula: "Mg",
    family: MaterialFamily::Magnesium,
    density_kg_m3: 1830.0,
    young_modulus_pa: 45.0e9,
    poisson_ratio: 0.35,
    yield_strength_pa: 285.0e6,
    ultimate_strength_pa: 365.0e6,
    thermal_conductivity_w_mk: 117.0,
    thermal_expansion_per_k: 26.0e-6,
    specific_heat_j_kgk: 1000.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 380.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 88.0,
    cost_eur_kg: 9.0,
});
