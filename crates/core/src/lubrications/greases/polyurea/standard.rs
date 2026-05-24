use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static POLYUREA_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Polyurea Grease NLGI 2",
    thickener: "Polyurea",
    family: GreaseFamily::Polyurea,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 100.0,
    density_kg_m3: 890.0,
    dropping_point_k: 533.0,
    operating_temp_min_k: 233.0,
    operating_temp_max_k: 453.0,
    penetration_worked_dmm: 280.0,
    friction_coefficient: 0.10,
    weld_point_n: 1960.0,
});
