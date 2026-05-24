use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static PTFE_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "PTFE Grease NLGI 2",
    thickener: "PTFE (Polytetrafluoroethylene)",
    family: GreaseFamily::Ptfe,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 100.0,
    density_kg_m3: 1080.0,
    dropping_point_k: f64::INFINITY,
    operating_temp_min_k: 213.0,
    operating_temp_max_k: 533.0,
    penetration_worked_dmm: 280.0,
    friction_coefficient: 0.05,
    weld_point_n: 1177.0,
});
