use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::Shorty,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 46.0,
        collector_diameter_mm: 74.0,
        catalyst_count: 6,
        back_pressure_kpa: 17.0,
        ceramic_coated: false,
    }
}

pub fn cast_iron() -> ExhaustSystem {
    ExhaustSystem {
        material: ExhaustMaterial::CastIron,
        header_primary_mm: 44.0,
        back_pressure_kpa: 20.0,
        ..standard()
    }
}
