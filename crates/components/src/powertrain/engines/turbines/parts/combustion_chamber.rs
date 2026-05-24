#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombustorType {
    Annular,
    CanAnnular,
    Can,
    ReverseFlow,
}

#[derive(Debug, Clone)]
pub struct CombustionChamber {
    pub combustor_type: CombustorType,
    pub fuel_flow_kg_h: f64,
    pub chamber_pressure_bar: f64,
    pub max_temperature_c: f64,
    pub fuel_air_ratio: f64,
    pub igniter_count: u8,
    pub liner_material_tbc: bool,
}

impl CombustionChamber {
    pub fn annular(chamber_pressure_bar: f64, max_temperature_c: f64) -> Self {
        Self {
            combustor_type: CombustorType::Annular,
            fuel_flow_kg_h: 2_400.0,
            chamber_pressure_bar,
            max_temperature_c,
            fuel_air_ratio: 0.025,
            igniter_count: 2,
            liner_material_tbc: true,
        }
    }

    pub fn can_annular(chamber_pressure_bar: f64, max_temperature_c: f64, can_count: u8) -> Self {
        Self {
            combustor_type: CombustorType::CanAnnular,
            fuel_flow_kg_h: can_count as f64 * 300.0,
            chamber_pressure_bar,
            max_temperature_c,
            fuel_air_ratio: 0.023,
            igniter_count: can_count,
            liner_material_tbc: true,
        }
    }

    pub fn reverse_flow(chamber_pressure_bar: f64, max_temperature_c: f64) -> Self {
        Self {
            combustor_type: CombustorType::ReverseFlow,
            fuel_flow_kg_h: 800.0,
            chamber_pressure_bar,
            max_temperature_c,
            fuel_air_ratio: 0.021,
            igniter_count: 2,
            liner_material_tbc: false,
        }
    }
}
