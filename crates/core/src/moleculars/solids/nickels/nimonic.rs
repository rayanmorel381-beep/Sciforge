use std::sync::LazyLock;
use sciforge_hub::prelude::constants::elements::by_symbol;
use super::super::{Material, MaterialFamily};

pub static NIMONIC_80A: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ni"),
    name: "Nimonic 80A",
    formula: "Ni",
    family: MaterialFamily::Nickel,
    density_kg_m3: 8190.0,
    young_modulus_pa: 222.0e9,
    poisson_ratio: 0.30,
    yield_strength_pa: 780.0e6,
    ultimate_strength_pa: 1250.0e6,
    thermal_conductivity_w_mk: 11.0,
    thermal_expansion_per_k: 12.7e-6,
    specific_heat_j_kgk: 448.0,
    max_service_temp_k: 1090.0,
    fatigue_strength_coeff_pa: 1600.0e6,
    fatigue_strength_exponent: -0.07,
    hardness_hv: 360.0,
    cost_eur_kg: 55.0,
});

pub static NIMONIC_90: LazyLock<Material> = LazyLock::new(|| Material {
    element: by_symbol("Ni"),
    name: "Nimonic 90",
    formula: "Ni",
    family: MaterialFamily::Nickel,
    density_kg_m3: 8180.0,
    young_modulus_pa: 213.0e9,
    poisson_ratio: 0.30,
    yield_strength_pa: 810.0e6,
    ultimate_strength_pa: 1240.0e6,
    thermal_conductivity_w_mk: 12.0,
    thermal_expansion_per_k: 12.7e-6,
    specific_heat_j_kgk: 460.0,
    max_service_temp_k: 1196.0,
    fatigue_strength_coeff_pa: 1650.0e6,
    fatigue_strength_exponent: -0.07,
    hardness_hv: 365.0,
    cost_eur_kg: 60.0,
});
