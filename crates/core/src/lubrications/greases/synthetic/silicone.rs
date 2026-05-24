use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static SILICONE_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Silicone Grease NLGI 2",
    thickener: "Amorphous Silica",
    family: GreaseFamily::Silicone,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 200.0,
    density_kg_m3: 970.0,
    dropping_point_k: f64::INFINITY,
    operating_temp_min_k: 193.0,
    operating_temp_max_k: 533.0,
    penetration_worked_dmm: 285.0,
    friction_coefficient: 0.15,
    weld_point_n: 785.0,
});
