use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::XPipe,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 52.0,
        collector_diameter_mm: 84.0,
        catalyst_count: 2,
        back_pressure_kpa: 7.0,
        ceramic_coated: true,
    }
}
