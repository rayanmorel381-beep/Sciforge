use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static JET_B: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Jet B",
    formula: "Kerosene-naphtha blend",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 775.0,
    dynamic_viscosity_pa_s_ref: 1.25e-3,
    bulk_modulus_pa: 1.42e9,
    specific_heat_j_kgk: 2050.0,
    thermal_conductivity_w_mk: 0.125,
    surface_tension_n_m: 0.023,
    vapor_pressure_pa_ref: 5000.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 188.15,
    flash_point_k: 248.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.005,
});
