use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static STEEL_NITRIDED_32CRMOV13: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Fe"),
    name: "32CrMoV13 nitrided steel",
    formula: "Fe",
    family: MaterialFamily::Iron,
    density_kg_m3: 7850.0,
    young_modulus_pa: 210.0e9,
    poisson_ratio: 0.29,
    yield_strength_pa: 1050.0e6,
    ultimate_strength_pa: 1250.0e6,
    thermal_conductivity_w_mk: 35.0,
    thermal_expansion_per_k: 12.5e-6,
    specific_heat_j_kgk: 477.0,
    max_service_temp_k: 800.0,
    fatigue_strength_coeff_pa: 1700.0e6,
    fatigue_strength_exponent: -0.08,
    hardness_hv: 950.0,
    cost_eur_kg: 8.0,
});
