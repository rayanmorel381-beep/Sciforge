use std::sync::LazyLock;

use sciforge_hub::prelude::meteorology::ocean::seawater_density;
use crate::moleculars::{Liquid, LiquidFamily};

pub static WATER: LazyLock<Liquid> = LazyLock::new(|| Liquid {
    name: "Fresh Water",
    formula: "H2O",
    family: LiquidFamily::Water,
    density_kg_m3_ref: seawater_density(20.0, 0.0),
    dynamic_viscosity_pa_s_ref: 1.002e-3,
    bulk_modulus_pa: 2.15e9,
    specific_heat_j_kgk: 4182.0,
    thermal_conductivity_w_mk: 0.598,
    surface_tension_n_m: 0.0728,
    vapor_pressure_pa_ref: 2339.0,
    temperature_ref_k: 293.15,
    viscosity_index: 0.0,
    pour_point_k: 273.15,
    flash_point_k: 0.0,
    shear_stability_index: 0.0,
    friction_coefficient: 0.01,
});
