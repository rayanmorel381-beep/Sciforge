use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static CAST_IRON_GREY: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Fe"),
    name: "Grey cast iron",
    formula: "Fe",
    family: MaterialFamily::Iron,
    density_kg_m3: 7150.0,
    young_modulus_pa: 110.0e9,
    poisson_ratio: 0.26,
    yield_strength_pa: 250.0e6,
    ultimate_strength_pa: 350.0e6,
    thermal_conductivity_w_mk: 50.0,
    thermal_expansion_per_k: 11.0e-6,
    specific_heat_j_kgk: 460.0,
    max_service_temp_k: 750.0,
    fatigue_strength_coeff_pa: 480.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 215.0,
    cost_eur_kg: 1.5,
});

pub static CAST_IRON_NODULAR_SG: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Fe"),
    name: "Nodular SG iron",
    formula: "Fe",
    family: MaterialFamily::Iron,
    density_kg_m3: 7100.0,
    young_modulus_pa: 170.0e9,
    poisson_ratio: 0.27,
    yield_strength_pa: 380.0e6,
    ultimate_strength_pa: 600.0e6,
    thermal_conductivity_w_mk: 36.0,
    thermal_expansion_per_k: 11.5e-6,
    specific_heat_j_kgk: 461.0,
    max_service_temp_k: 750.0,
    fatigue_strength_coeff_pa: 700.0e6,
    fatigue_strength_exponent: -0.09,
    hardness_hv: 240.0,
    cost_eur_kg: 2.0,
});

pub static CGI_400: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Fe"),
    name: "CGI 400 vermicular iron",
    formula: "Fe",
    family: MaterialFamily::Iron,
    density_kg_m3: 7100.0,
    young_modulus_pa: 145.0e9,
    poisson_ratio: 0.26,
    yield_strength_pa: 300.0e6,
    ultimate_strength_pa: 400.0e6,
    thermal_conductivity_w_mk: 38.0,
    thermal_expansion_per_k: 12.0e-6,
    specific_heat_j_kgk: 460.0,
    max_service_temp_k: 750.0,
    fatigue_strength_coeff_pa: 600.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 220.0,
    cost_eur_kg: 2.5,
});
