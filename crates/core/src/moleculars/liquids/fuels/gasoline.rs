use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static GASOLINE: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Gasoline",
    formula: "C8H18 approx",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 745.0,
    dynamic_viscosity_pa_s_ref: 0.0006,
    bulk_modulus_pa: 1.35e9,
    specific_heat_j_kgk: 2220.0,
    thermal_conductivity_w_mk: 0.145,
    surface_tension_n_m: 0.022,
    vapor_pressure_pa_ref: 50000.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 193.15,
    flash_point_k: 228.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.005,
});
