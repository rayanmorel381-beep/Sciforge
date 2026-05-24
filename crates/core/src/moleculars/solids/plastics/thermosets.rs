use std::sync::LazyLock;
use super::super::{Material, MaterialFamily};

pub static EPOXY: LazyLock<Material> = LazyLock::new(|| Material {
    element: None,
    name: "Epoxy resin",
    formula: "C21H25ClO5",
    family: MaterialFamily::Plastic,
    density_kg_m3: 1200.0,
    young_modulus_pa: 3.5e9,
    poisson_ratio: 0.35,
    yield_strength_pa: 60.0e6,
    ultimate_strength_pa: 90.0e6,
    thermal_conductivity_w_mk: 0.20,
    thermal_expansion_per_k: 65.0e-6,
    specific_heat_j_kgk: 1100.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 110.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 18.0,
    cost_eur_kg: 8.0,
});

pub static PHENOLIC: LazyLock<Material> = LazyLock::new(|| Material {
    element: None,
    name: "Phenolic Bakelite",
    formula: "C7H8O2",
    family: MaterialFamily::Plastic,
    density_kg_m3: 1300.0,
    young_modulus_pa: 5.2e9,
    poisson_ratio: 0.35,
    yield_strength_pa: 50.0e6,
    ultimate_strength_pa: 70.0e6,
    thermal_conductivity_w_mk: 0.22,
    thermal_expansion_per_k: 30.0e-6,
    specific_heat_j_kgk: 1400.0,
    max_service_temp_k: 473.0,
    fatigue_strength_coeff_pa: 90.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 25.0,
    cost_eur_kg: 5.0,
});
