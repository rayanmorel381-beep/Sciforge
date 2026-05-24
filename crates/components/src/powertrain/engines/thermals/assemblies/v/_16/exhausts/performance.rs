use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 54.0,
        collector_diameter_mm: 90.0,
        catalyst_count: 4,
        back_pressure_kpa: 10.0,
        ceramic_coated: true,
    }
}
