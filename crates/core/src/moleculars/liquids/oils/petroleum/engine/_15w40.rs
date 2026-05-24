use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static ENGINE_OIL_SAE_15W40: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Engine Oil SAE 15W-40",
    formula: "Hydrocarbon+additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 868.0,
    dynamic_viscosity_pa_s_ref: 0.14,
    bulk_modulus_pa: 1.76e9,
    specific_heat_j_kgk: 2040.0,
    thermal_conductivity_w_mk: 0.142,
    surface_tension_n_m: 0.032,
    vapor_pressure_pa_ref: 35.0,
    temperature_ref_k: 313.15,
    viscosity_index: 148.0,
    pour_point_k: 238.15,
    flash_point_k: 503.15,
    shear_stability_index: 20.0,
    friction_coefficient: 0.07,
});
