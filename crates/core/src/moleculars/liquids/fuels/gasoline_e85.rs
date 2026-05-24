use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static GASOLINE_E85: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "E85 Fuel",
    formula: "85% ethanol blend",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 780.0,
    dynamic_viscosity_pa_s_ref: 1.05e-3,
    bulk_modulus_pa: 1.15e9,
    specific_heat_j_kgk: 2360.0,
    thermal_conductivity_w_mk: 0.165,
    surface_tension_n_m: 0.022,
    vapor_pressure_pa_ref: 23000.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 183.15,
    flash_point_k: 282.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.005,
});
