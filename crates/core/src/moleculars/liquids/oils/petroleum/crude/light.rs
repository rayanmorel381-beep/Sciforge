use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static CRUDE_OIL_LIGHT: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Light Crude Oil",
    formula: "Hydrocarbon mix",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 830.0,
    dynamic_viscosity_pa_s_ref: 7.5e-3,
    bulk_modulus_pa: 1.45e9,
    specific_heat_j_kgk: 2100.0,
    thermal_conductivity_w_mk: 0.13,
    surface_tension_n_m: 0.028,
    vapor_pressure_pa_ref: 5000.0,
    temperature_ref_k: 293.15,
    viscosity_index: 45.0,
    pour_point_k: 238.15,
    flash_point_k: 288.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.04,
});
