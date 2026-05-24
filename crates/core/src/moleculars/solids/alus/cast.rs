use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static AC4B: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Al"),
    name: "AC4B cast aluminium",
    formula: "Al",
    family: MaterialFamily::Aluminum,
    density_kg_m3: 2700.0,
    young_modulus_pa: 71.0e9,
    poisson_ratio: 0.33,
    yield_strength_pa: 200.0e6,
    ultimate_strength_pa: 280.0e6,
    thermal_conductivity_w_mk: 155.0,
    thermal_expansion_per_k: 22.0e-6,
    specific_heat_j_kgk: 900.0,
    max_service_temp_k: 523.0,
    fatigue_strength_coeff_pa: 360.0e6,
    fatigue_strength_exponent: -0.11,
    hardness_hv: 90.0,
    cost_eur_kg: 3.5,
});

pub static A356: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Al"),
    name: "A356 cast aluminium",
    formula: "Al",
    family: MaterialFamily::Aluminum,
    density_kg_m3: 2680.0,
    young_modulus_pa: 72.4e9,
    poisson_ratio: 0.33,
    yield_strength_pa: 205.0e6,
    ultimate_strength_pa: 275.0e6,
    thermal_conductivity_w_mk: 151.0,
    thermal_expansion_per_k: 21.5e-6,
    specific_heat_j_kgk: 963.0,
    max_service_temp_k: 473.0,
    fatigue_strength_coeff_pa: 350.0e6,
    fatigue_strength_exponent: -0.11,
    hardness_hv: 85.0,
    cost_eur_kg: 3.2,
});

pub static ALSI_AC319: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Al"),
    name: "AC319 AlSi block",
    formula: "Al",
    family: MaterialFamily::Aluminum,
    density_kg_m3: 2790.0,
    young_modulus_pa: 74.0e9,
    poisson_ratio: 0.33,
    yield_strength_pa: 165.0e6,
    ultimate_strength_pa: 250.0e6,
    thermal_conductivity_w_mk: 109.0,
    thermal_expansion_per_k: 21.4e-6,
    specific_heat_j_kgk: 880.0,
    max_service_temp_k: 523.0,
    fatigue_strength_coeff_pa: 320.0e6,
    fatigue_strength_exponent: -0.11,
    hardness_hv: 95.0,
    cost_eur_kg: 3.0,
});
