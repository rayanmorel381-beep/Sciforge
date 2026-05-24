use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static BIODIESEL_B100: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Biodiesel B100",
    formula: "FAME mix",
    family: LiquidFamily::Fuel,
    density_kg_m3_ref: 880.0,
    dynamic_viscosity_pa_s_ref: 4.5e-3,
    bulk_modulus_pa: 1.75e9,
    specific_heat_j_kgk: 2000.0,
    thermal_conductivity_w_mk: 0.17,
    surface_tension_n_m: 0.028,
    vapor_pressure_pa_ref: 200.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 268.15,
    flash_point_k: 403.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.02,
});
