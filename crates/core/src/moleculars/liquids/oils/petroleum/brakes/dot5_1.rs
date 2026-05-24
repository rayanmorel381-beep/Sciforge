use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static BRAKE_FLUID_DOT5_1: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Brake Fluid DOT 5.1",
    formula: "High-performance glycol ethers",
    family: LiquidFamily::Hydraulic,
    density_kg_m3_ref: 1055.0,
    dynamic_viscosity_pa_s_ref: 0.0055,
    bulk_modulus_pa: 1.95e9,
    specific_heat_j_kgk: 2420.0,
    thermal_conductivity_w_mk: 0.25,
    surface_tension_n_m: 0.034,
    vapor_pressure_pa_ref: 220.0,
    temperature_ref_k: 313.15,
    viscosity_index: 1200.0,
    pour_point_k: 203.15,
    flash_point_k: 443.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
