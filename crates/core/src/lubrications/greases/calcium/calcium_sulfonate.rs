use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static CALCIUM_SULFONATE_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Calcium Sulfonate Complex Grease NLGI 2",
    thickener: "Calcium Sulfonate Complex",
    family: GreaseFamily::CalciumSulfonate,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 220.0,
    density_kg_m3: 920.0,
    dropping_point_k: 593.0,
    operating_temp_min_k: 233.0,
    operating_temp_max_k: 453.0,
    penetration_worked_dmm: 265.0,
    friction_coefficient: 0.10,
    weld_point_n: 3924.0,
});
