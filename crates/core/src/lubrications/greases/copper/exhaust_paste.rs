use std::sync::LazyLock;
use crate::lubrications::{Grease, GreaseFamily};

pub static COPPER_EXHAUST_PASTE: LazyLock<Grease> = LazyLock::new(|| Grease {
    name: "Copper Exhaust Manifold Paste",
    thickener: "Copper Powder in Synthetic Carrier",
    family: GreaseFamily::Copper,
    nlgi_grade: 1,
    base_oil_viscosity_mm2_s: 130.0,
    density_kg_m3: 1320.0,
    dropping_point_k: 773.0,
    operating_temp_min_k: 233.0,
    operating_temp_max_k: 1373.0,
    penetration_worked_dmm: 320.0,
    friction_coefficient: 0.12,
    weld_point_n: 2943.0,
});
