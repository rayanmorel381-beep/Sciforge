use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static AUTOMATIC_TRANS_FLUID_CVTF: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "CVT Fluid",
    formula: "Hydrocarbon+friction modifiers",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 845.0,
    dynamic_viscosity_pa_s_ref: 0.032,
    bulk_modulus_pa: 1.65e9,
    specific_heat_j_kgk: 2050.0,
    thermal_conductivity_w_mk: 0.14,
    surface_tension_n_m: 0.03,
    vapor_pressure_pa_ref: 55.0,
    temperature_ref_k: 313.15,
    viscosity_index: 170.0,
    pour_point_k: 218.15,
    flash_point_k: 453.15,
    shear_stability_index: 25.0,
    friction_coefficient: 0.04,
});
