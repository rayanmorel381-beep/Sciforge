use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static GEAR_OIL_85W90: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Gear Oil SAE 85W-90",
    formula: "Hydrocarbon+EP additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 878.0,
    dynamic_viscosity_pa_s_ref: 0.162,
    bulk_modulus_pa: 1.74e9,
    specific_heat_j_kgk: 2040.0,
    thermal_conductivity_w_mk: 0.139,
    surface_tension_n_m: 0.032,
    vapor_pressure_pa_ref: 35.0,
    temperature_ref_k: 313.15,
    viscosity_index: 120.0,
    pour_point_k: 248.15,
    flash_point_k: 463.15,
    shear_stability_index: 15.0,
    friction_coefficient: 0.06,
});
