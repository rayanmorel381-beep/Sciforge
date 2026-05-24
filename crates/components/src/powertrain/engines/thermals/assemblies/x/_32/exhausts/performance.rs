use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 58.0,
        collector_diameter_mm: 96.0,
        catalyst_count: 16,
        back_pressure_kpa: 12.0,
        ceramic_coated: true,
    }
}
