use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 62.0,
        collector_diameter_mm: 106.0,
        catalyst_count: 4,
        back_pressure_kpa: 9.0,
        ceramic_coated: true,
    }
}

pub fn long_tube() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 64.0,
        collector_diameter_mm: 112.0,
        back_pressure_kpa: 7.0,
        ..standard()
    }
}
