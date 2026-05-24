use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::Titanium,
        header_primary_mm: 58.0,
        collector_diameter_mm: 96.0,
        catalyst_count: 0,
        back_pressure_kpa: 4.5,
        ceramic_coated: true,
    }
}

pub fn stainless() -> ExhaustSystem {
    ExhaustSystem {
        material: ExhaustMaterial::StainlessSteel,
        back_pressure_kpa: 5.5,
        ..standard()
    }
}

pub fn long_tube_titanium() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 60.0,
        collector_diameter_mm: 100.0,
        back_pressure_kpa: 4.0,
        ..standard()
    }
}
