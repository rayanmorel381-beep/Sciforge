use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static WATERLESS_COOLANT: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Waterless Coolant",
    formula: "Glycol blend",
    family: LiquidFamily::Coolant,
    density_kg_m3_ref: 1050.0,
    dynamic_viscosity_pa_s_ref: 6.0e-3,
    bulk_modulus_pa: 2.1e9,
    specific_heat_j_kgk: 2800.0,
    thermal_conductivity_w_mk: 0.29,
    surface_tension_n_m: 0.043,
    vapor_pressure_pa_ref: 500.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 218.15,
    flash_point_k: 0.0,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
