use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::Shorty,
        material: ExhaustMaterial::MildSteel,
        header_primary_mm: 42.0,
        collector_diameter_mm: 63.0,
        catalyst_count: 2,
        back_pressure_kpa: 16.0,
        ceramic_coated: false,
    }
}

pub fn stainless() -> ExhaustSystem {
    ExhaustSystem {
        material: ExhaustMaterial::StainlessSteel,
        back_pressure_kpa: 14.0,
        ..standard()
    }
}
