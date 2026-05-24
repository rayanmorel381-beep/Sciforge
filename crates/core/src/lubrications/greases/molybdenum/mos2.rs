use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static MOLYBDENUM_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Molybdenum Disulfide Grease NLGI 2",
    thickener: "Lithium with MoS2",
    family: GreaseFamily::Molybdenum,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 150.0,
    density_kg_m3: 1000.0,
    dropping_point_k: 453.0,
    operating_temp_min_k: 233.0,
    operating_temp_max_k: 393.0,
    penetration_worked_dmm: 280.0,
    friction_coefficient: 0.07,
    weld_point_n: 2943.0,
});
