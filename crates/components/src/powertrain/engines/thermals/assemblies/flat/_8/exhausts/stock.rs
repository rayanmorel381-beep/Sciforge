use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::Shorty,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 44.0,
        collector_diameter_mm: 70.0,
        catalyst_count: 4,
        back_pressure_kpa: 16.0,
        ceramic_coated: false,
    }
}

pub fn cast_iron() -> ExhaustSystem {
    ExhaustSystem {
        material: ExhaustMaterial::CastIron,
        header_primary_mm: 42.0,
        back_pressure_kpa: 19.0,
        ..standard()
    }
}

pub fn mild_steel() -> ExhaustSystem {
    ExhaustSystem {
        material: ExhaustMaterial::MildSteel,
        back_pressure_kpa: 17.5,
        ..standard()
    }
}
