use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static STAINLESS_304: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Fe"),
    name: "304 austenitic stainless",
    formula: "Fe",
    family: MaterialFamily::Iron,
    density_kg_m3: 8000.0,
    young_modulus_pa: 193.0e9,
    poisson_ratio: 0.29,
    yield_strength_pa: 215.0e6,
    ultimate_strength_pa: 505.0e6,
    thermal_conductivity_w_mk: 16.2,
    thermal_expansion_per_k: 17.3e-6,
    specific_heat_j_kgk: 500.0,
    max_service_temp_k: 1144.0,
    fatigue_strength_coeff_pa: 800.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 200.0,
    cost_eur_kg: 4.5,
});

pub static STAINLESS_316: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Fe"),
    name: "316L marine stainless",
    formula: "Fe",
    family: MaterialFamily::Iron,
    density_kg_m3: 8000.0,
    young_modulus_pa: 193.0e9,
    poisson_ratio: 0.29,
    yield_strength_pa: 290.0e6,
    ultimate_strength_pa: 580.0e6,
    thermal_conductivity_w_mk: 16.3,
    thermal_expansion_per_k: 16.0e-6,
    specific_heat_j_kgk: 500.0,
    max_service_temp_k: 1144.0,
    fatigue_strength_coeff_pa: 850.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 217.0,
    cost_eur_kg: 6.5,
});

pub static STAINLESS_410: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Fe"),
    name: "410 martensitic stainless",
    formula: "Fe",
    family: MaterialFamily::Iron,
    density_kg_m3: 7800.0,
    young_modulus_pa: 200.0e9,
    poisson_ratio: 0.29,
    yield_strength_pa: 415.0e6,
    ultimate_strength_pa: 690.0e6,
    thermal_conductivity_w_mk: 24.9,
    thermal_expansion_per_k: 9.9e-6,
    specific_heat_j_kgk: 460.0,
    max_service_temp_k: 922.0,
    fatigue_strength_coeff_pa: 950.0e6,
    fatigue_strength_exponent: -0.09,
    hardness_hv: 280.0,
    cost_eur_kg: 5.0,
});
