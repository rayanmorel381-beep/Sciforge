use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static ENGINE_OIL_SAE_10W40: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Engine Oil SAE 10W-40",
    formula: "Hydrocarbon+additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 860.0,
    dynamic_viscosity_pa_s_ref: 0.11,
    bulk_modulus_pa: 1.74e9,
    specific_heat_j_kgk: 2060.0,
    thermal_conductivity_w_mk: 0.143,
    surface_tension_n_m: 0.031,
    vapor_pressure_pa_ref: 40.0,
    temperature_ref_k: 313.15,
    viscosity_index: 155.0,
    pour_point_k: 233.15,
    flash_point_k: 498.15,
    shear_stability_index: 22.0,
    friction_coefficient: 0.07,
});
