use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::XPipe,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 62.0,
        collector_diameter_mm: 112.0,
        catalyst_count: 4,
        back_pressure_kpa: 6.0,
        ceramic_coated: true,
    }
}
