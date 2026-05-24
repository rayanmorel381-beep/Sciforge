#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressorType {
    Axial,
    Centrifugal,
    Mixed,
}

#[derive(Debug, Clone)]
pub struct TurbineCompressor {
    pub compressor_type: CompressorType,
    pub stage_count: u8,
    pub pressure_ratio: f64,
    pub mass_flow_kg_s: f64,
    pub polytropic_efficiency: f64,
    pub tip_speed_m_s: f64,
}

impl TurbineCompressor {
    pub fn axial(stage_count: u8, pressure_ratio: f64, mass_flow_kg_s: f64) -> Self {
        Self {
            compressor_type: CompressorType::Axial,
            stage_count,
            pressure_ratio,
            mass_flow_kg_s,
            polytropic_efficiency: 0.88,
            tip_speed_m_s: 380.0,
        }
    }

    pub fn centrifugal(pressure_ratio: f64, mass_flow_kg_s: f64) -> Self {
        Self {
            compressor_type: CompressorType::Centrifugal,
            stage_count: 1,
            pressure_ratio,
            mass_flow_kg_s,
            polytropic_efficiency: 0.82,
            tip_speed_m_s: 480.0,
        }
    }

    pub fn mixed(stage_count: u8, pressure_ratio: f64, mass_flow_kg_s: f64) -> Self {
        Self {
            compressor_type: CompressorType::Mixed,
            stage_count,
            pressure_ratio,
            mass_flow_kg_s,
            polytropic_efficiency: 0.85,
            tip_speed_m_s: 420.0,
        }
    }
}
