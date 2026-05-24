use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::TriY,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 50.0,
        collector_diameter_mm: 84.0,
        catalyst_count: 2,
        back_pressure_kpa: 7.5,
        ceramic_coated: true,
    }
}
