use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::Titanium,
        header_primary_mm: 56.0,
        collector_diameter_mm: 94.0,
        catalyst_count: 0,
        back_pressure_kpa: 5.0,
        ceramic_coated: true,
    }
}

pub fn long_tube() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 58.0,
        collector_diameter_mm: 98.0,
        back_pressure_kpa: 4.3,
        ..standard()
    }
}
