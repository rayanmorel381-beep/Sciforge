use crate::powertrain::engines::thermals::parts::exhausts::{ExhaustMaterial, ExhaustSystem, HeaderType};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::Titanium,
        header_primary_mm: 40.0,
        collector_diameter_mm: 66.0,
        catalyst_count: 0,
        back_pressure_kpa: 3.2,
        ceramic_coated: true,
    }
}

pub fn long_tube() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 42.0,
        collector_diameter_mm: 70.0,
        back_pressure_kpa: 2.7,
        ..standard()
    }
}
