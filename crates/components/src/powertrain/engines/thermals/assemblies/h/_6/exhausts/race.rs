use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::Titanium,
        header_primary_mm: 48.0,
        collector_diameter_mm: 78.0,
        catalyst_count: 0,
        back_pressure_kpa: 2.8,
        ceramic_coated: true,
    }
}

pub fn stainless() -> ExhaustSystem {
    ExhaustSystem {
        material: ExhaustMaterial::StainlessSteel,
        back_pressure_kpa: 3.8,
        ..standard()
    }
}

pub fn long_tube_titanium() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 50.0,
        collector_diameter_mm: 82.0,
        back_pressure_kpa: 2.3,
        ..standard()
    }
}
