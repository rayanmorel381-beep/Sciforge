use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static ETHYLENE_GLYCOL_WATER_50: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Ethylene Glycol Coolant 50/50",
    formula: "C2H6O2+H2O",
    family: LiquidFamily::Coolant,
    density_kg_m3_ref: 1065.0,
    dynamic_viscosity_pa_s_ref: 3.5e-3,
    bulk_modulus_pa: 2.3e9,
    specific_heat_j_kgk: 3400.0,
    thermal_conductivity_w_mk: 0.40,
    surface_tension_n_m: 0.048,
    vapor_pressure_pa_ref: 1800.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 238.15,
    flash_point_k: 0.0,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
