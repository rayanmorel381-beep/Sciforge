use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static CFRP_T700: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("C"),
    name: "T700 carbon/epoxy",
    formula: "C",
    family: MaterialFamily::Composite,
    density_kg_m3: 1600.0,
    young_modulus_pa: 135.0e9,
    poisson_ratio: 0.30,
    yield_strength_pa: 2300.0e6,
    ultimate_strength_pa: 2550.0e6,
    thermal_conductivity_w_mk: 7.0,
    thermal_expansion_per_k: -0.4e-6,
    specific_heat_j_kgk: 1100.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 1800.0e6,
    fatigue_strength_exponent: -0.07,
    hardness_hv: 0.0,
    cost_eur_kg: 80.0,
});

pub static CFRP_T800: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("C"),
    name: "T800 carbon/epoxy",
    formula: "C",
    family: MaterialFamily::Composite,
    density_kg_m3: 1620.0,
    young_modulus_pa: 165.0e9,
    poisson_ratio: 0.30,
    yield_strength_pa: 2900.0e6,
    ultimate_strength_pa: 3100.0e6,
    thermal_conductivity_w_mk: 7.0,
    thermal_expansion_per_k: -0.4e-6,
    specific_heat_j_kgk: 1100.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 2200.0e6,
    fatigue_strength_exponent: -0.07,
    hardness_hv: 0.0,
    cost_eur_kg: 120.0,
});

pub static CFRP_M55J: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("C"),
    name: "M55J ultra-high-modulus carbon",
    formula: "C",
    family: MaterialFamily::Composite,
    density_kg_m3: 1660.0,
    young_modulus_pa: 290.0e9,
    poisson_ratio: 0.30,
    yield_strength_pa: 3000.0e6,
    ultimate_strength_pa: 3200.0e6,
    thermal_conductivity_w_mk: 25.0,
    thermal_expansion_per_k: -1.1e-6,
    specific_heat_j_kgk: 1100.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 2400.0e6,
    fatigue_strength_exponent: -0.06,
    hardness_hv: 0.0,
    cost_eur_kg: 350.0,
});
