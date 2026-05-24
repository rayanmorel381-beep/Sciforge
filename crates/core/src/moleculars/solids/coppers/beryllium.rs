use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static CUBE2: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Cu"),
    name: "CuBe2 beryllium copper",
    formula: "Cu",
    family: MaterialFamily::Copper,
    density_kg_m3: 8250.0,
    young_modulus_pa: 131.0e9,
    poisson_ratio: 0.30,
    yield_strength_pa: 1100.0e6,
    ultimate_strength_pa: 1380.0e6,
    thermal_conductivity_w_mk: 105.0,
    thermal_expansion_per_k: 17.0e-6,
    specific_heat_j_kgk: 420.0,
    max_service_temp_k: 573.0,
    fatigue_strength_coeff_pa: 1400.0e6,
    fatigue_strength_exponent: -0.08,
    hardness_hv: 410.0,
    cost_eur_kg: 95.0,
});
