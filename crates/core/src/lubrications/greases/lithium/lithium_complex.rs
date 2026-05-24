use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static LITHIUM_COMPLEX_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Lithium Complex Grease NLGI 2",
    thickener: "Lithium Complex Soap",
    family: GreaseFamily::LithiumComplex,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 100.0,
    density_kg_m3: 885.0,
    dropping_point_k: 533.0,
    operating_temp_min_k: 233.0,
    operating_temp_max_k: 453.0,
    penetration_worked_dmm: 275.0,
    friction_coefficient: 0.11,
    weld_point_n: 1960.0,
});
