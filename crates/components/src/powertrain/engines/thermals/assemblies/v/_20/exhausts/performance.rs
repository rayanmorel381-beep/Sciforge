use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 58.0,
        collector_diameter_mm: 98.0,
        catalyst_count: 4,
        back_pressure_kpa: 9.5,
        ceramic_coated: true,
    }
}

pub fn long_tube() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 60.0,
        collector_diameter_mm: 104.0,
        back_pressure_kpa: 7.5,
        ..standard()
    }
}
