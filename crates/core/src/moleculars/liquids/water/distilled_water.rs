use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static DISTILLED_WATER: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Distilled Water",
    formula: "H2O",
    family: LiquidFamily::Water,
    density_kg_m3_ref: 998.0,
    dynamic_viscosity_pa_s_ref: 1.00e-3,
    bulk_modulus_pa: 2.2e9,
    specific_heat_j_kgk: 4180.0,
    thermal_conductivity_w_mk: 0.60,
    surface_tension_n_m: 0.072,
    vapor_pressure_pa_ref: 2339.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 273.15,
    flash_point_k: 0.0,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
