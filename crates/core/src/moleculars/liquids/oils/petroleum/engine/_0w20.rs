use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static ENGINE_OIL_SAE_0W20: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Engine Oil SAE 0W-20",
    formula: "Hydrocarbon+additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 842.0,
    dynamic_viscosity_pa_s_ref: 0.045,
    bulk_modulus_pa: 1.68e9,
    specific_heat_j_kgk: 2120.0,
    thermal_conductivity_w_mk: 0.144,
    surface_tension_n_m: 0.030,
    vapor_pressure_pa_ref: 60.0,
    temperature_ref_k: 313.15,
    viscosity_index: 165.0,
    pour_point_k: 223.15,
    flash_point_k: 493.15,
    shear_stability_index: 20.0,
    friction_coefficient: 0.05,
});
