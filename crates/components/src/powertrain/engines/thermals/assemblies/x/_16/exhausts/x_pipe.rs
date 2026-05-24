use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::TriY,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 50.0,
        collector_diameter_mm: 82.0,
        catalyst_count: 4,
        back_pressure_kpa: 8.5,
        ceramic_coated: true,
    }
}
