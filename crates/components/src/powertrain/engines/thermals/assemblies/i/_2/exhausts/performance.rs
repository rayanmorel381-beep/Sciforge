use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::EqualLength,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 40.0,
        collector_diameter_mm: 60.0,
        catalyst_count: 1,
        back_pressure_kpa: 10.0,
        ceramic_coated: false,
    }
}

pub fn ceramic_coated() -> ExhaustSystem {
    ExhaustSystem {
        ceramic_coated: true,
        back_pressure_kpa: 8.5,
        ..standard()
    }
}

pub fn long_tube() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::LongTube,
        header_primary_mm: 42.0,
        collector_diameter_mm: 62.0,
        back_pressure_kpa: 8.0,
        ..standard()
    }
}

pub fn long_tube_ceramic() -> ExhaustSystem {
    ExhaustSystem {
        ceramic_coated: true,
        back_pressure_kpa: 7.0,
        ..long_tube()
    }
}
