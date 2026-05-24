use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static AUTOMATIC_TRANS_FLUID_DEXRON_III: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "ATF Dexron III",
    formula: "Hydrocarbon+friction modifiers",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 850.0,
    dynamic_viscosity_pa_s_ref: 0.041,
    bulk_modulus_pa: 1.67e9,
    specific_heat_j_kgk: 2040.0,
    thermal_conductivity_w_mk: 0.14,
    surface_tension_n_m: 0.03,
    vapor_pressure_pa_ref: 51.0,
    temperature_ref_k: 313.15,
    viscosity_index: 160.0,
    pour_point_k: 223.15,
    flash_point_k: 453.15,
    shear_stability_index: 22.0,
    friction_coefficient: 0.04,
});
