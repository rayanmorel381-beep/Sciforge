use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static SODIUM_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Sodium Grease NLGI 2",
    thickener: "Sodium Soap",
    family: GreaseFamily::Sodium,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 95.0,
    density_kg_m3: 890.0,
    dropping_point_k: 448.0,
    operating_temp_min_k: 253.0,
    operating_temp_max_k: 393.0,
    penetration_worked_dmm: 280.0,
    friction_coefficient: 0.13,
    weld_point_n: 980.0,
});
