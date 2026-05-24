use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::HPipe,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 48.0,
        collector_diameter_mm: 76.0,
        catalyst_count: 2,
        back_pressure_kpa: 11.5,
        ceramic_coated: false,
    }
}

pub fn ceramic_coated() -> ExhaustSystem {
    ExhaustSystem {
        ceramic_coated: true,
        back_pressure_kpa: 10.0,
        ..standard()
    }
}
