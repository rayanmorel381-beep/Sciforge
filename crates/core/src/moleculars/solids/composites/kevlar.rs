use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static KEVLAR_29_EPOXY: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("C"),
    name: "Kevlar 29 / epoxy",
    formula: "C",
    family: MaterialFamily::Composite,
    density_kg_m3: 1380.0,
    young_modulus_pa: 70.0e9,
    poisson_ratio: 0.36,
    yield_strength_pa: 2500.0e6,
    ultimate_strength_pa: 2900.0e6,
    thermal_conductivity_w_mk: 0.04,
    thermal_expansion_per_k: -2.0e-6,
    specific_heat_j_kgk: 1420.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 1900.0e6,
    fatigue_strength_exponent: -0.08,
    hardness_hv: 0.0,
    cost_eur_kg: 50.0,
});

pub static KEVLAR_49_EPOXY: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("C"),
    name: "Kevlar 49 / epoxy",
    formula: "C",
    family: MaterialFamily::Composite,
    density_kg_m3: 1440.0,
    young_modulus_pa: 124.0e9,
    poisson_ratio: 0.36,
    yield_strength_pa: 3000.0e6,
    ultimate_strength_pa: 3600.0e6,
    thermal_conductivity_w_mk: 0.04,
    thermal_expansion_per_k: -2.0e-6,
    specific_heat_j_kgk: 1420.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 2400.0e6,
    fatigue_strength_exponent: -0.07,
    hardness_hv: 0.0,
    cost_eur_kg: 75.0,
});
