use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static METHANOL: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Methanol",
    formula: "CH4O",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 792.0,
    dynamic_viscosity_pa_s_ref: 0.59e-3,
    bulk_modulus_pa: 1.04e9,
    specific_heat_j_kgk: 2510.0,
    thermal_conductivity_w_mk: 0.203,
    surface_tension_n_m: 0.0226,
    vapor_pressure_pa_ref: 12700.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 175.55,
    flash_point_k: 284.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.005,
});
