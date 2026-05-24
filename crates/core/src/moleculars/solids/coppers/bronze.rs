use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static BRONZE_CUSN8: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Cu"),
    name: "CuSn8 phosphor bronze",
    formula: "Cu",
    family: MaterialFamily::Copper,
    density_kg_m3: 8800.0,
    young_modulus_pa: 110.0e9,
    poisson_ratio: 0.34,
    yield_strength_pa: 380.0e6,
    ultimate_strength_pa: 580.0e6,
    thermal_conductivity_w_mk: 62.0,
    thermal_expansion_per_k: 18.0e-6,
    specific_heat_j_kgk: 380.0,
    max_service_temp_k: 473.0,
    fatigue_strength_coeff_pa: 700.0e6,
    fatigue_strength_exponent: -0.09,
    hardness_hv: 220.0,
    cost_eur_kg: 11.0,
});

pub static BRONZE_MANGANESE: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Cu"),
    name: "Manganese bronze",
    formula: "Cu",
    family: MaterialFamily::Copper,
    density_kg_m3: 8300.0,
    young_modulus_pa: 100.0e9,
    poisson_ratio: 0.34,
    yield_strength_pa: 280.0e6,
    ultimate_strength_pa: 500.0e6,
    thermal_conductivity_w_mk: 87.0,
    thermal_expansion_per_k: 19.0e-6,
    specific_heat_j_kgk: 380.0,
    max_service_temp_k: 473.0,
    fatigue_strength_coeff_pa: 580.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 165.0,
    cost_eur_kg: 9.0,
});
