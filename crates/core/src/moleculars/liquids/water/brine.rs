use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static BRINE: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Brine",
    formula: "H2O+NaCl",
    family: LiquidFamily::Water,
    density_kg_m3_ref: 1200.0,
    dynamic_viscosity_pa_s_ref: 1.6e-3,
    bulk_modulus_pa: 2.6e9,
    specific_heat_j_kgk: 3500.0,
    thermal_conductivity_w_mk: 0.52,
    surface_tension_n_m: 0.078,
    vapor_pressure_pa_ref: 1200.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 261.15,
    flash_point_k: 0.0,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
