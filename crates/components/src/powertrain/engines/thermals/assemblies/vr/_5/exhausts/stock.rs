use crate::powertrain::engines::thermals::parts::exhausts::{ExhaustMaterial, ExhaustSystem, HeaderType};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::Shorty,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 34.0,
        collector_diameter_mm: 55.0,
        catalyst_count: 2,
        back_pressure_kpa: 13.5,
        ceramic_coated: false,
    }
}
