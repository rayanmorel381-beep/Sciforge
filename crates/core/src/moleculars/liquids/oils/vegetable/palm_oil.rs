use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static PALM_OIL: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Palm Oil",
    formula: "Triglycerides",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 890.0,
    dynamic_viscosity_pa_s_ref: 0.043,
    bulk_modulus_pa: 1.55e9,
    specific_heat_j_kgk: 1950.0,
    thermal_conductivity_w_mk: 0.17,
    surface_tension_n_m: 0.031,
    vapor_pressure_pa_ref: 10.0,
    temperature_ref_k: 323.15,
    viscosity_index: 190.0,
    pour_point_k: 298.15,
    flash_point_k: 553.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.08,
});
