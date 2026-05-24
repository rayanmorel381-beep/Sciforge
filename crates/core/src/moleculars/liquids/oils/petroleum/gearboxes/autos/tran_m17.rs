use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static AUTOMATIC_TRANS_FLUID_TRAN_M17: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Automatic Transmission Fluid TRAN M17",
    formula: "Hydrocarbon+friction modifiers",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 855.0,
    dynamic_viscosity_pa_s_ref: 0.045,
    bulk_modulus_pa: 1.69e9,
    specific_heat_j_kgk: 2035.0,
    thermal_conductivity_w_mk: 0.139,
    surface_tension_n_m: 0.031,
    vapor_pressure_pa_ref: 48.0,
    temperature_ref_k: 313.15,
    viscosity_index: 162.0,
    pour_point_k: 218.15,
    flash_point_k: 453.15,
    shear_stability_index: 20.0,
    friction_coefficient: 0.04,
});
