use crate::powertrain::engines::thermals::parts::exhausts::{
    ExhaustMaterial, ExhaustSystem, HeaderType,
};

pub fn standard() -> ExhaustSystem {
    ExhaustSystem {
        header_type: HeaderType::XPipe,
        material: ExhaustMaterial::StainlessSteel,
        header_primary_mm: 60.0,
        collector_diameter_mm: 106.0,
        catalyst_count: 2,
        back_pressure_kpa: 6.5,
        ceramic_coated: true,
    }
}
