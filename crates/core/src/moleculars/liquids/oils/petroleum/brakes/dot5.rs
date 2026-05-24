use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static BRAKE_FLUID_DOT5: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Brake Fluid DOT 5",
    formula: "Silicone-based",
    family: LiquidFamily::Hydraulic,
    density_kg_m3_ref: 960.0,
    dynamic_viscosity_pa_s_ref: 0.013,
    bulk_modulus_pa: 1.6e9,
    specific_heat_j_kgk: 1500.0,
    thermal_conductivity_w_mk: 0.15,
    surface_tension_n_m: 0.021,
    vapor_pressure_pa_ref: 40.0,
    temperature_ref_k: 313.15,
    viscosity_index: 800.0,
    pour_point_k: 208.15,
    flash_point_k: 453.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
