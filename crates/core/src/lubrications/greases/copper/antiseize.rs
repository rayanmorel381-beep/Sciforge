use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static COPPER_ANTISEIZE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Copper Anti-Seize Compound",
    thickener: "Copper Flakes in Mineral Oil",
    family: GreaseFamily::Copper,
    nlgi_grade: 1,
    base_oil_viscosity_mm2_s: 120.0,
    density_kg_m3: 1250.0,
    dropping_point_k: 723.0,
    operating_temp_min_k: 233.0,
    operating_temp_max_k: 1273.0,
    penetration_worked_dmm: 310.0,
    friction_coefficient: 0.13,
    weld_point_n: 2452.0,
});
