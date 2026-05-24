use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static BIODIESEL_B20: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Biodiesel B20",
    formula: "20% FAME blend",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 844.0,
    dynamic_viscosity_pa_s_ref: 3.1e-3,
    bulk_modulus_pa: 1.66e9,
    specific_heat_j_kgk: 2030.0,
    thermal_conductivity_w_mk: 0.145,
    surface_tension_n_m: 0.027,
    vapor_pressure_pa_ref: 350.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 255.15,
    flash_point_k: 325.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
