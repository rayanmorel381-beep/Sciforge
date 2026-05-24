use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static ALUMINUM_COMPLEX_GREASE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Aluminum Complex Grease NLGI 2",
    thickener: "Aluminum Complex Soap",
    family: GreaseFamily::Aluminum,
    nlgi_grade: 2,
    base_oil_viscosity_mm2_s: 100.0,
    density_kg_m3: 875.0,
    dropping_point_k: 523.0,
    operating_temp_min_k: 243.0,
    operating_temp_max_k: 423.0,
    penetration_worked_dmm: 280.0,
    friction_coefficient: 0.12,
    weld_point_n: 1765.0,
});
