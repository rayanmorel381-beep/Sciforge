use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static GEAR_OIL_75W110: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Gear Oil SAE 75W-110",
    formula: "Hydrocarbon+EP additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 878.0,
    dynamic_viscosity_pa_s_ref: 0.182,
    bulk_modulus_pa: 1.74e9,
    specific_heat_j_kgk: 2040.0,
    thermal_conductivity_w_mk: 0.139,
    surface_tension_n_m: 0.032,
    vapor_pressure_pa_ref: 36.0,
    temperature_ref_k: 313.15,
    viscosity_index: 155.0,
    pour_point_k: 228.15,
    flash_point_k: 463.15,
    shear_stability_index: 15.0,
    friction_coefficient: 0.06,
});
