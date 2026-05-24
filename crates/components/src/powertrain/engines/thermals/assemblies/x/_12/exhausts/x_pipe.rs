use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::TriY,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 48.0,
        collector_diameter_mm: 78.0,
        catalyst_count: 3,
        back_pressure_kpa: 8.0,
        ceramic_coated: true,
    }
}
