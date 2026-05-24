use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static GEAR_OIL_80W: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Gear Oil SAE 80W",
    formula: "Hydrocarbon+EP additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 865.0,
    dynamic_viscosity_pa_s_ref: 0.101,
    bulk_modulus_pa: 1.71e9,
    specific_heat_j_kgk: 2055.0,
    thermal_conductivity_w_mk: 0.141,
    surface_tension_n_m: 0.031,
    vapor_pressure_pa_ref: 41.0,
    temperature_ref_k: 313.15,
    viscosity_index: 88.0,
    pour_point_k: 246.15,
    flash_point_k: 453.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.07,
});
