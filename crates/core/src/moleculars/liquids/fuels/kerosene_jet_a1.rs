use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static KEROSENE_JET_A1: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Kerosene Jet A-1",
    formula: "C12H26 approx",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 804.0,
    dynamic_viscosity_pa_s_ref: 1.64e-3,
    bulk_modulus_pa: 1.5e9,
    specific_heat_j_kgk: 2010.0,
    thermal_conductivity_w_mk: 0.13,
    surface_tension_n_m: 0.025,
    vapor_pressure_pa_ref: 1000.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 220.15,
    flash_point_k: 311.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.005,
});
