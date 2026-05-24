use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static COPPER_C110: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Cu"),
    name: "C110 ETP copper",
    formula: "Cu",
    family: MaterialFamily::Copper,
    density_kg_m3: 8940.0,
    young_modulus_pa: 117.0e9,
    poisson_ratio: 0.34,
    yield_strength_pa: 70.0e6,
    ultimate_strength_pa: 220.0e6,
    thermal_conductivity_w_mk: 391.0,
    thermal_expansion_per_k: 16.9e-6,
    specific_heat_j_kgk: 385.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 280.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 50.0,
    cost_eur_kg: 8.5,
});
