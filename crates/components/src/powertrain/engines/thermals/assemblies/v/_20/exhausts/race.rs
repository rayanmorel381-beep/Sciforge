use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::Titanium,
        header_primary_mm: 60.0,
        collector_diameter_mm: 108.0,
        catalyst_count: 0,
        back_pressure_kpa: 3.0,
        ceramic_coated: true,
    }
}

pub fn long_tube() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 62.0,
        collector_diameter_mm: 112.0,
        back_pressure_kpa: 2.5,
        ..standard()
    }
}
