use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static AUTOMATIC_TRANS_FLUID_DEXRON_II: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "ATF Dexron II",
    formula: "Hydrocarbon+friction modifiers",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 853.0,
    dynamic_viscosity_pa_s_ref: 0.046,
    bulk_modulus_pa: 1.68e9,
    specific_heat_j_kgk: 2035.0,
    thermal_conductivity_w_mk: 0.139,
    surface_tension_n_m: 0.03,
    vapor_pressure_pa_ref: 49.0,
    temperature_ref_k: 313.15,
    viscosity_index: 155.0,
    pour_point_k: 228.15,
    flash_point_k: 453.15,
    shear_stability_index: 25.0,
    friction_coefficient: 0.04,
});
