use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static GEAR_OIL_75W: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Gear Oil SAE 75W",
    formula: "Hydrocarbon+EP additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 860.0,
    dynamic_viscosity_pa_s_ref: 0.083,
    bulk_modulus_pa: 1.70e9,
    specific_heat_j_kgk: 2065.0,
    thermal_conductivity_w_mk: 0.141,
    surface_tension_n_m: 0.031,
    vapor_pressure_pa_ref: 44.0,
    temperature_ref_k: 313.15,
    viscosity_index: 90.0,
    pour_point_k: 228.15,
    flash_point_k: 453.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.07,
});
