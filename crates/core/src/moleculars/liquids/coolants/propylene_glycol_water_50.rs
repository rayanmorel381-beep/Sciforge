use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static PROPYLENE_GLYCOL_WATER_50: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Propylene Glycol Coolant 50/50",
    formula: "C3H8O2+H2O",
    family: LiquidFamily::Coolant,
    density_kg_m3_ref: 1035.0,
    dynamic_viscosity_pa_s_ref: 4.2e-3,
    bulk_modulus_pa: 2.25e9,
    specific_heat_j_kgk: 3600.0,
    thermal_conductivity_w_mk: 0.38,
    surface_tension_n_m: 0.046,
    vapor_pressure_pa_ref: 1700.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 243.15,
    flash_point_k: 0.0,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
