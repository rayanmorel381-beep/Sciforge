use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static DIESEL: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Diesel",
    formula: "C12H23 approx",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 832.0,
    dynamic_viscosity_pa_s_ref: 2.6e-3,
    bulk_modulus_pa: 1.6e9,
    specific_heat_j_kgk: 2050.0,
    thermal_conductivity_w_mk: 0.13,
    surface_tension_n_m: 0.026,
    vapor_pressure_pa_ref: 500.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 258.15,
    flash_point_k: 328.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
