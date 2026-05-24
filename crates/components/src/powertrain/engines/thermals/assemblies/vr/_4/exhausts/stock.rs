use crate::powertrain::engines::thermals::parts::exhausts::{ExhaustMaterial, ExhaustSystem, HeaderType};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::Shorty,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 32.0,
        collector_diameter_mm: 52.0,
        catalyst_count: 1,
        back_pressure_kpa: 14.0,
        ceramic_coated: false,
    }
}
