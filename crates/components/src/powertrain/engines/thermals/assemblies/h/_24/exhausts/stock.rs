use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::Shorty,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 52.0,
        collector_diameter_mm: 85.0,
        catalyst_count: 12,
        back_pressure_kpa: 19.5,
        ceramic_coated: false,
    }
}

pub fn cast_iron() -> ExhaustSystem {
    ExhaustSystem {
        material: ExhaustMaterial::CastIron,
        header_primary_mm: 50.0,
        back_pressure_kpa: 22.5,
        ..standard()
    }
}
