use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::Shorty,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 38.0,
        collector_diameter_mm: 60.0,
        catalyst_count: 2,
        back_pressure_kpa: 13.0,
        ceramic_coated: false,
    }
}
