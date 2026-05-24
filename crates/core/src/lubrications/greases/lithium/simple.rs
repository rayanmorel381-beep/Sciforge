use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static LITHIUM_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Lithium Grease NLGI 2",
    thickener: "Lithium 12-Hydroxystearate",
    family: GreaseFamily::Lithium,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 100.0,
    density_kg_m3: 880.0,
    dropping_point_k: 463.0,
    operating_temp_min_k: 243.0,
    operating_temp_max_k: 403.0,
    penetration_worked_dmm: 280.0,
    friction_coefficient: 0.12,
    weld_point_n: 1570.0,
});
