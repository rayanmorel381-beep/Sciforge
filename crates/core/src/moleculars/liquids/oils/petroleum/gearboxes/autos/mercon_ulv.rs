use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static AUTOMATIC_TRANS_FLUID_MERCON_ULV: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "ATF Mercon ULV",
    formula: "Hydrocarbon+friction modifiers",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 840.0,
    dynamic_viscosity_pa_s_ref: 0.024,
    bulk_modulus_pa: 1.63e9,
    specific_heat_j_kgk: 2060.0,
    thermal_conductivity_w_mk: 0.142,
    surface_tension_n_m: 0.028,
    vapor_pressure_pa_ref: 61.0,
    temperature_ref_k: 313.15,
    viscosity_index: 180.0,
    pour_point_k: 213.15,
    flash_point_k: 453.15,
    shear_stability_index: 15.0,
    friction_coefficient: 0.03,
});
