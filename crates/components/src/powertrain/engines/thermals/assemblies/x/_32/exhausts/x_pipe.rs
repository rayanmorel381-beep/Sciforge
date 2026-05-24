use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::TriY,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 58.0,
        collector_diameter_mm: 94.0,
        catalyst_count: 8,
        back_pressure_kpa: 10.5,
        ceramic_coated: true,
    }
}
