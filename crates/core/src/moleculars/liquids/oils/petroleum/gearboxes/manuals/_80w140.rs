use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static GEAR_OIL_80W140: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Gear Oil SAE 80W-140",
    formula: "Hydrocarbon+EP additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 888.0,
    dynamic_viscosity_pa_s_ref: 0.276,
    bulk_modulus_pa: 1.77e9,
    specific_heat_j_kgk: 2025.0,
    thermal_conductivity_w_mk: 0.138,
    surface_tension_n_m: 0.033,
    vapor_pressure_pa_ref: 30.0,
    temperature_ref_k: 313.15,
    viscosity_index: 135.0,
    pour_point_k: 243.15,
    flash_point_k: 463.15,
    shear_stability_index: 15.0,
    friction_coefficient: 0.06,
});
