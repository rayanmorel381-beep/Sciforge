use std::sync::LazyLock;

use sciforge_hub::prelude::meteorology::ocean::seawater_density;
use crate::moleculars::{Liquid, LiquidFamily};

pub static SEAWATER: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Seawater",
    formula: "H2O+salts",
    family: LiquidFamily::Water,
    density_kg_m3_ref: seawater_density(15.0, 35.0),
    dynamic_viscosity_pa_s_ref: 1.19e-3,
    bulk_modulus_pa: 2.34e9,
    specific_heat_j_kgk: 3990.0,
    thermal_conductivity_w_mk: 0.58,
    surface_tension_n_m: 0.074,
    vapor_pressure_pa_ref: 1705.0,
    temperature_ref_k: 288.15,
    viscosity_index: 0.0,
    pour_point_k: 271.15,
    flash_point_k: 0.0,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
