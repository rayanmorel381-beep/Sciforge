use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 52.0,
        collector_diameter_mm: 86.0,
        catalyst_count: 10,
        back_pressure_kpa: 10.5,
        ceramic_coated: true,
    }
}
