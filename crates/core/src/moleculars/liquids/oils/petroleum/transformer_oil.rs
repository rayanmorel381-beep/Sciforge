use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static TRANSFORMER_OIL: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Transformer Oil",
    formula: "Highly refined mineral oil",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 890.0,
    dynamic_viscosity_pa_s_ref: 0.022,
    bulk_modulus_pa: 1.66e9,
    specific_heat_j_kgk: 1880.0,
    thermal_conductivity_w_mk: 0.126,
    surface_tension_n_m: 0.040,
    vapor_pressure_pa_ref: 10.0,
    temperature_ref_k: 313.15,
    viscosity_index: 80.0,
    pour_point_k: 223.15,
    flash_point_k: 423.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.03,
});
