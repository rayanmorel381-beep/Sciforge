use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::YPipe,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 44.0,
        collector_diameter_mm: 70.0,
        catalyst_count: 2,
        back_pressure_kpa: 11.0,
        ceramic_coated: false,
    }
}

pub fn ceramic_coated() -> ExhaustSystem {
    ExhaustSystem {
        ceramic_coated: true,
        back_pressure_kpa: 9.5,
        ..standard()
    }
}
