use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 44.0,
        collector_diameter_mm: 73.0,
        catalyst_count: 6,
        back_pressure_kpa: 15.0,
        ceramic_coated: false,
    }
}
