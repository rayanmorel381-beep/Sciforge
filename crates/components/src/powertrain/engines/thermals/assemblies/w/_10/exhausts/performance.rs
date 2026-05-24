use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 48.0,
        collector_diameter_mm: 80.0,
        catalyst_count: 4,
        back_pressure_kpa: 9.2,
        ceramic_coated: true,
    }
}
