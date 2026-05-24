use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static CRUDE_OIL_HEAVY: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Heavy Crude Oil",
    formula: "Hydrocarbon mix",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 970.0,
    dynamic_viscosity_pa_s_ref: 0.85,
    bulk_modulus_pa: 1.65e9,
    specific_heat_j_kgk: 1900.0,
    thermal_conductivity_w_mk: 0.12,
    surface_tension_n_m: 0.031,
    vapor_pressure_pa_ref: 200.0,
    temperature_ref_k: 293.15,
    viscosity_index: 15.0,
    pour_point_k: 268.15,
    flash_point_k: 343.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.07,
});
