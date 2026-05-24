use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static CANOLA_OIL: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Canola Oil",
    formula: "Triglycerides",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 915.0,
    dynamic_viscosity_pa_s_ref: 0.062,
    bulk_modulus_pa: 1.6e9,
    specific_heat_j_kgk: 1960.0,
    thermal_conductivity_w_mk: 0.17,
    surface_tension_n_m: 0.032,
    vapor_pressure_pa_ref: 10.0,
    temperature_ref_k: 293.15,
    viscosity_index: 210.0,
    pour_point_k: 258.15,
    flash_point_k: 543.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.09,
});
