use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static MAR_M247: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ni"),
    name: "Mar-M247 superalloy",
    formula: "Ni",
    family: MaterialFamily::Nickel,
    density_kg_m3: 8540.0,
    young_modulus_pa: 218.0e9,
    poisson_ratio: 0.30,
    yield_strength_pa: 815.0e6,
    ultimate_strength_pa: 965.0e6,
    thermal_conductivity_w_mk: 11.5,
    thermal_expansion_per_k: 12.0e-6,
    specific_heat_j_kgk: 420.0,
    max_service_temp_k: 1273.0,
    fatigue_strength_coeff_pa: 1500.0e6,
    fatigue_strength_exponent: -0.07,
    hardness_hv: 380.0,
    cost_eur_kg: 120.0,
});
