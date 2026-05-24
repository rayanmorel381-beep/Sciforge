use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static AL_7075_T6: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Al"),
    name: "7075-T6 billet aluminium",
    formula: "Al",
    family: MaterialFamily::Aluminum,
    density_kg_m3: 2810.0,
    young_modulus_pa: 71.7e9,
    poisson_ratio: 0.33,
    yield_strength_pa: 503.0e6,
    ultimate_strength_pa: 572.0e6,
    thermal_conductivity_w_mk: 130.0,
    thermal_expansion_per_k: 23.4e-6,
    specific_heat_j_kgk: 960.0,
    max_service_temp_k: 423.0,
    fatigue_strength_coeff_pa: 700.0e6,
    fatigue_strength_exponent: -0.10,
    hardness_hv: 175.0,
    cost_eur_kg: 12.0,
});
