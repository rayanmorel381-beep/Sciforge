use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static BRAKE_FLUID_DOT2: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Brake Fluid DOT 2",
    formula: "Castor oil/alcohol blend",
    family: LiquidFamily::Hydraulic,
    density_kg_m3_ref: 940.0,
    dynamic_viscosity_pa_s_ref: 0.012,
    bulk_modulus_pa: 1.75e9,
    specific_heat_j_kgk: 1850.0,
    thermal_conductivity_w_mk: 0.17,
    surface_tension_n_m: 0.033,
    vapor_pressure_pa_ref: 120.0,
    temperature_ref_k: 313.15,
    viscosity_index: 1000.0,
    pour_point_k: 208.15,
    flash_point_k: 383.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
