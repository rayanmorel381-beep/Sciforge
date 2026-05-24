use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static AUTOMATIC_TRANS_FLUID_EV_REDUCTION_GEAR: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "EV Reduction Gear Fluid",
    formula: "Synthetic hydrocarbon+e-drive additives",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 836.0,
    dynamic_viscosity_pa_s_ref: 0.022,
    bulk_modulus_pa: 1.61e9,
    specific_heat_j_kgk: 2070.0,
    thermal_conductivity_w_mk: 0.143,
    surface_tension_n_m: 0.028,
    vapor_pressure_pa_ref: 64.0,
    temperature_ref_k: 313.15,
    viscosity_index: 200.0,
    pour_point_k: 213.15,
    flash_point_k: 463.15,
    shear_stability_index: 10.0,
    friction_coefficient: 0.02,
});
