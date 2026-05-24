use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static PHOSPHATE_ESTER_HFD: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Phosphate Ester HFD",
    formula: "Organophosphate esters",
    family: LiquidFamily::Hydraulic,
    density_kg_m3_ref: 1150.0,
    dynamic_viscosity_pa_s_ref: 0.042,
    bulk_modulus_pa: 1.85e9,
    specific_heat_j_kgk: 1700.0,
    thermal_conductivity_w_mk: 0.14,
    surface_tension_n_m: 0.037,
    vapor_pressure_pa_ref: 20.0,
    temperature_ref_k: 313.15,
    viscosity_index: 70.0,
    pour_point_k: 233.15,
    flash_point_k: 533.15,
    shear_stability_index: 0.0,
    friction_coefficient: 0.05,
});
