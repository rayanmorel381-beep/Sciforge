use std::sync::LazyLock;
use super::super::{Material, MaterialFamily};

pub static E_GLASS_EPOXY: LazyLock<Material> = LazyLock::new(|| Material {
    element: None,
    name: "E-glass/epoxy",
    formula: "SiO2",
    family: MaterialFamily::Composite,
    density_kg_m3: 2000.0,
    young_modulus_pa: 39.0e9,
    poisson_ratio: 0.28,
    yield_strength_pa: 1000.0e6,
    ultimate_strength_pa: 1200.0e6,
    thermal_conductivity_w_mk: 0.4,
    thermal_expansion_per_k: 8.6e-6,
    specific_heat_j_kgk: 850.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 700.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 0.0,
    cost_eur_kg: 8.0,
});

pub static S_GLASS_EPOXY: LazyLock<Material> = LazyLock::new(|| Material {
    element: None,
    name: "S-glass/epoxy",
    formula: "SiO2",
    family: MaterialFamily::Composite,
    density_kg_m3: 1980.0,
    young_modulus_pa: 52.0e9,
    poisson_ratio: 0.28,
    yield_strength_pa: 1500.0e6,
    ultimate_strength_pa: 1700.0e6,
    thermal_conductivity_w_mk: 0.4,
    thermal_expansion_per_k: 5.6e-6,
    specific_heat_j_kgk: 850.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 1100.0e6,
    fatigue_strength_exponent: -0.09,
    hardness_hv: 0.0,
    cost_eur_kg: 25.0,
});
