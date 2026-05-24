use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static BENTONE_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Bentone Clay Grease NLGI 2",
    thickener: "Organoclay (Bentone)",
    family: GreaseFamily::Bentone,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 150.0,
    density_kg_m3: 940.0,
    dropping_point_k: f64::INFINITY,
    operating_temp_min_k: 233.0,
    operating_temp_max_k: 473.0,
    penetration_worked_dmm: 280.0,
    friction_coefficient: 0.13,
    weld_point_n: 1570.0,
});
