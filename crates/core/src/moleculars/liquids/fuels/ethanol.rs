use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static ETHANOL: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Ethanol",
    formula: "C2H6O",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 789.0,
    dynamic_viscosity_pa_s_ref: 1.20e-3,
    bulk_modulus_pa: 1.09e9,
    specific_heat_j_kgk: 2440.0,
    thermal_conductivity_w_mk: 0.171,
    surface_tension_n_m: 0.0223,
    vapor_pressure_pa_ref: 5870.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 159.15,
    flash_point_k: 286.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.005,
});
