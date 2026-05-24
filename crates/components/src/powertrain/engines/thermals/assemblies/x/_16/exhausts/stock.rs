use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 46.0,
        collector_diameter_mm: 77.0,
        catalyst_count: 8,
        back_pressure_kpa: 16.0,
        ceramic_coated: false,
    }
}
