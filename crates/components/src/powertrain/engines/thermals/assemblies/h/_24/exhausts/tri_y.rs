use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::TriY,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 54.0,
        collector_diameter_mm: 88.0,
        catalyst_count: 6,
        back_pressure_kpa: 12.5,
        ceramic_coated: false,
    }
}
