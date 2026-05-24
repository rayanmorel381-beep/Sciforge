use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static TURBINE_OIL_ISO68: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Turbine Oil ISO VG 68",
    formula: "Refined mineral oil",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 872.0,
    dynamic_viscosity_pa_s_ref: 0.058,
    bulk_modulus_pa: 1.7e9,
    specific_heat_j_kgk: 1980.0,
    thermal_conductivity_w_mk: 0.138,
    surface_tension_n_m: 0.032,
    vapor_pressure_pa_ref: 30.0,
    temperature_ref_k: 313.15,
    viscosity_index: 105.0,
    pour_point_k: 233.15,
    flash_point_k: 493.15,
    shear_stability_index: 5.0,
    friction_coefficient: 0.05,
});
