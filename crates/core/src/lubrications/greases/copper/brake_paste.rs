use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static COPPER_BRAKE_PASTE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Copper Brake Assembly Paste",
    thickener: "Copper Flakes in Mineral Oil",
    family: GreaseFamily::Copper,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 110.0,
    density_kg_m3: 1180.0,
    dropping_point_k: 623.0,
    operating_temp_min_k: 233.0,
    operating_temp_max_k: 1073.0,
    penetration_worked_dmm: 290.0,
    friction_coefficient: 0.14,
    weld_point_n: 2060.0,
});
