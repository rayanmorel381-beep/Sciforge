use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static GEAR_OIL_80W90: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Gear Oil SAE 80W-90",
    formula: "Hydrocarbon+EP additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 875.0,
    dynamic_viscosity_pa_s_ref: 0.149,
    bulk_modulus_pa: 1.73e9,
    specific_heat_j_kgk: 2045.0,
    thermal_conductivity_w_mk: 0.14,
    surface_tension_n_m: 0.032,
    vapor_pressure_pa_ref: 37.0,
    temperature_ref_k: 313.15,
    viscosity_index: 130.0,
    pour_point_k: 243.15,
    flash_point_k: 463.15,
    shear_stability_index: 15.0,
    friction_coefficient: 0.06,
});
