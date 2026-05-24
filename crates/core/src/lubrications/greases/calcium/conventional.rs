use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static CALCIUM_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Calcium Grease NLGI 2",
    thickener: "Calcium Soap",
    family: GreaseFamily::Calcium,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 90.0,
    density_kg_m3: 900.0,
    dropping_point_k: 373.0,
    operating_temp_min_k: 243.0,
    operating_temp_max_k: 333.0,
    penetration_worked_dmm: 280.0,
    friction_coefficient: 0.14,
    weld_point_n: 980.0,
});
