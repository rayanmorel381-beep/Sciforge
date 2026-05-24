use crate::powertrain::engines::thermals::parts::exhausts::{ExhaustMaterial, ExhaustSystem, HeaderType};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::TriY,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 36.0,
        collector_diameter_mm: 58.0,
        catalyst_count: 1,
        back_pressure_kpa: 7.0,
        ceramic_coated: true,
    }
}
