use std::sync::LazyLock;

use crate::moleculars::{Liquid, LiquidFamily};

pub static COMPRESSOR_OIL_ISO100: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Compressor Oil ISO VG 100",
    formula: "Mineral/synthetic base oil",
    family: LiquidFamily::Oil,
    density_kg_m3_ref: 880.0,
    dynamic_viscosity_pa_s_ref: 0.095,
    bulk_modulus_pa: 1.72e9,
    specific_heat_j_kgk: 1970.0,
    thermal_conductivity_w_mk: 0.136,
    surface_tension_n_m: 0.033,
    vapor_pressure_pa_ref: 20.0,
    temperature_ref_k: 313.15,
    viscosity_index: 110.0,
    pour_point_k: 238.15,
    flash_point_k: 503.15,
    shear_stability_index: 5.0,
    friction_coefficient: 0.05,
});
