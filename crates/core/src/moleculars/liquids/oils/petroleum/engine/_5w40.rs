use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static ENGINE_OIL_SAE_5W40: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Engine Oil SAE 5W-40",
    formula: "Hydrocarbon+additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 852.0,
    dynamic_viscosity_pa_s_ref: 0.089,
    bulk_modulus_pa: 1.72e9,
    specific_heat_j_kgk: 2090.0,
    thermal_conductivity_w_mk: 0.144,
    surface_tension_n_m: 0.031,
    vapor_pressure_pa_ref: 45.0,
    temperature_ref_k: 313.15,
    viscosity_index: 163.0,
    pour_point_k: 228.15,
    flash_point_k: 498.15,
    shear_stability_index: 20.0,
    friction_coefficient: 0.06,
});
