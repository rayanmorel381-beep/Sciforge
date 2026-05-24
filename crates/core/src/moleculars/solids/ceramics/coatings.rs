use std::sync::LazyLock;
use super::super::{Material, MaterialFamily};

pub static ALUMINA_AL2O3: LazyLock<Material> = LazyLock::new(|| Material {
    element: None,
    name: "Alumina Al2O3 99.5%",
    formula: "Al2O3",
    family: MaterialFamily::Ceramic,
    density_kg_m3: 3960.0,
    young_modulus_pa: 380.0e9,
    poisson_ratio: 0.22,
    yield_strength_pa: 350.0e6,
    ultimate_strength_pa: 350.0e6,
    thermal_conductivity_w_mk: 30.0,
    thermal_expansion_per_k: 8.1e-6,
    specific_heat_j_kgk: 880.0,
    max_service_temp_k: 1923.0,
    fatigue_strength_coeff_pa: 500.0e6,
    fatigue_strength_exponent: -0.05,
    hardness_hv: 1500.0,
    cost_eur_kg: 25.0,
});

pub static TBC_YSZ: LazyLock<Material> = LazyLock::new(|| Material {
    element: None,
    name: "TBC YSZ thermal barrier",
    formula: "ZrO2",
    family: MaterialFamily::Ceramic,
    density_kg_m3: 5800.0,
    young_modulus_pa: 50.0e9,
    poisson_ratio: 0.25,
    yield_strength_pa: 250.0e6,
    ultimate_strength_pa: 250.0e6,
    thermal_conductivity_w_mk: 1.0,
    thermal_expansion_per_k: 11.0e-6,
    specific_heat_j_kgk: 500.0,
    max_service_temp_k: 1973.0,
    fatigue_strength_coeff_pa: 350.0e6,
    fatigue_strength_exponent: -0.06,
    hardness_hv: 950.0,
    cost_eur_kg: 75.0,
});
