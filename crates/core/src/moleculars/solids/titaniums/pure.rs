use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static CP_TI_GR2: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ti"),
    name: "CP titanium Grade 2",
    formula: "Ti",
    family: MaterialFamily::Titanium,
    density_kg_m3: 4510.0,
    young_modulus_pa: 105.0e9,
    poisson_ratio: 0.37,
    yield_strength_pa: 275.0e6,
    ultimate_strength_pa: 345.0e6,
    thermal_conductivity_w_mk: 16.4,
    thermal_expansion_per_k: 8.6e-6,
    specific_heat_j_kgk: 528.0,
    max_service_temp_k: 700.0,
    fatigue_strength_coeff_pa: 500.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 200.0,
    cost_eur_kg: 25.0,
});
