use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static ATF: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Automatic Transmission Fluid",
    formula: "Hydrocarbon+friction modifiers",
    family: LiquidFamily::Hydraulic,
    density_kg_m3_ref: 845.0,
    dynamic_viscosity_pa_s_ref: 0.034,
    bulk_modulus_pa: 1.58e9,
    specific_heat_j_kgk: 2100.0,
    thermal_conductivity_w_mk: 0.14,
    surface_tension_n_m: 0.030,
    vapor_pressure_pa_ref: 90.0,
    temperature_ref_k: 313.15,
    viscosity_index: 155.0,
    pour_point_k: 223.15,
    flash_point_k: 453.15,
    shear_stability_index: 20.0,
    friction_coefficient: 0.04,
});
