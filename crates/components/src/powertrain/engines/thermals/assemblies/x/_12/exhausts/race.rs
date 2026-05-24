use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::Titanium,
        header_primary_mm: 50.0,
        collector_diameter_mm: 83.0,
        catalyst_count: 0,
        back_pressure_kpa: 3.8,
        ceramic_coated: true,
    }
}

pub fn long_tube() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 52.0,
        collector_diameter_mm: 87.0,
        back_pressure_kpa: 3.0999999999999996,
        ..standard()
    }
}
