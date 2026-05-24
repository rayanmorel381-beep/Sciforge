use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static TI6AL4V_GR5: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ti"),
    name: "Ti-6Al-4V Grade 5",
    formula: "Ti",
    family: MaterialFamily::Titanium,
    density_kg_m3: 4430.0,
    young_modulus_pa: 114.0e9,
    poisson_ratio: 0.34,
    yield_strength_pa: 880.0e6,
    ultimate_strength_pa: 950.0e6,
    thermal_conductivity_w_mk: 6.7,
    thermal_expansion_per_k: 8.6e-6,
    specific_heat_j_kgk: 526.3,
    max_service_temp_k: 700.0,
    fatigue_strength_coeff_pa: 1100.0e6,
    fatigue_strength_exponent: -0.08,
    hardness_hv: 349.0,
    cost_eur_kg: 35.0,
});

pub static TI_GR9: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ti"),
    name: "Ti-3Al-2.5V Grade 9",
    formula: "Ti",
    family: MaterialFamily::Titanium,
    density_kg_m3: 4480.0,
    young_modulus_pa: 100.0e9,
    poisson_ratio: 0.34,
    yield_strength_pa: 620.0e6,
    ultimate_strength_pa: 720.0e6,
    thermal_conductivity_w_mk: 8.3,
    thermal_expansion_per_k: 9.3e-6,
    specific_heat_j_kgk: 540.0,
    max_service_temp_k: 700.0,
    fatigue_strength_coeff_pa: 850.0e6,
    fatigue_strength_exponent: -0.08,
    hardness_hv: 280.0,
    cost_eur_kg: 32.0,
});

pub static TI_BETA_C: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ti"),
    name: "Beta-C Ti-3Al-8V-6Cr-4Mo-4Zr",
    formula: "Ti",
    family: MaterialFamily::Titanium,
    density_kg_m3: 4810.0,
    young_modulus_pa: 86.0e9,
    poisson_ratio: 0.34,
    yield_strength_pa: 1200.0e6,
    ultimate_strength_pa: 1300.0e6,
    thermal_conductivity_w_mk: 7.6,
    thermal_expansion_per_k: 9.4e-6,
    specific_heat_j_kgk: 502.0,
    max_service_temp_k: 700.0,
    fatigue_strength_coeff_pa: 1500.0e6,
    fatigue_strength_exponent: -0.07,
    hardness_hv: 420.0,
    cost_eur_kg: 80.0,
});
