use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static INCONEL_718: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ni"),
    name: "Inconel 718",
    formula: "Ni",
    family: MaterialFamily::Nickel,
    density_kg_m3: 8190.0,
    young_modulus_pa: 200.0e9,
    poisson_ratio: 0.29,
    yield_strength_pa: 1100.0e6,
    ultimate_strength_pa: 1370.0e6,
    thermal_conductivity_w_mk: 11.4,
    thermal_expansion_per_k: 13.0e-6,
    specific_heat_j_kgk: 435.0,
    max_service_temp_k: 980.0,
    fatigue_strength_coeff_pa: 1800.0e6,
    fatigue_strength_exponent: -0.07,
    hardness_hv: 420.0,
    cost_eur_kg: 45.0,
});

pub static INCONEL_625: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ni"),
    name: "Inconel 625",
    formula: "Ni",
    family: MaterialFamily::Nickel,
    density_kg_m3: 8440.0,
    young_modulus_pa: 207.5e9,
    poisson_ratio: 0.31,
    yield_strength_pa: 480.0e6,
    ultimate_strength_pa: 930.0e6,
    thermal_conductivity_w_mk: 9.8,
    thermal_expansion_per_k: 12.8e-6,
    specific_heat_j_kgk: 410.0,
    max_service_temp_k: 1255.0,
    fatigue_strength_coeff_pa: 1300.0e6,
    fatigue_strength_exponent: -0.08,
    hardness_hv: 290.0,
    cost_eur_kg: 50.0,
});

pub static INCONEL_713C: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ni"),
    name: "Inconel 713C turbine",
    formula: "Ni",
    family: MaterialFamily::Nickel,
    density_kg_m3: 7910.0,
    young_modulus_pa: 206.0e9,
    poisson_ratio: 0.28,
    yield_strength_pa: 750.0e6,
    ultimate_strength_pa: 850.0e6,
    thermal_conductivity_w_mk: 10.9,
    thermal_expansion_per_k: 11.6e-6,
    specific_heat_j_kgk: 420.0,
    max_service_temp_k: 1255.0,
    fatigue_strength_coeff_pa: 1400.0e6,
    fatigue_strength_exponent: -0.08,
    hardness_hv: 350.0,
    cost_eur_kg: 60.0,
});
