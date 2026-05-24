use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static AUTOMATIC_TRANS_FLUID_DEXRON_VI: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "ATF Dexron VI",
    formula: "Hydrocarbon+friction modifiers",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 844.0,
    dynamic_viscosity_pa_s_ref: 0.031,
    bulk_modulus_pa: 1.65e9,
    specific_heat_j_kgk: 2050.0,
    thermal_conductivity_w_mk: 0.141,
    surface_tension_n_m: 0.029,
    vapor_pressure_pa_ref: 56.0,
    temperature_ref_k: 313.15,
    viscosity_index: 175.0,
    pour_point_k: 218.15,
    flash_point_k: 458.15,
    shear_stability_index: 20.0,
    friction_coefficient: 0.03,
});
