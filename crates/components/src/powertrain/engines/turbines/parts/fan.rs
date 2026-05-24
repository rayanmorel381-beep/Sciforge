#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanType {
    Single,
    Geared,
    OpenRotor,
}

#[derive(Debug, Clone)]
pub struct Fan {
    pub fan_type: FanType,
    pub diameter_m: f64,
    pub stage_count: u8,
    pub pressure_ratio: f64,
    pub mass_flow_kg_s: f64,
    pub tip_speed_m_s: f64,
    pub polytropic_efficiency: f64,
}

impl Fan {
    pub fn high_bypass(diameter_m: f64, mass_flow_kg_s: f64) -> Self {
        Self {
            fan_type: FanType::Single,
            diameter_m,
            stage_count: 1,
            pressure_ratio: 1.65,
            mass_flow_kg_s,
            tip_speed_m_s: 420.0,
            polytropic_efficiency: 0.90,
        }
    }

    pub fn geared(diameter_m: f64, mass_flow_kg_s: f64, gear_ratio: f64) -> Self {
        let tip_speed = 420.0 / gear_ratio;
        Self {
            fan_type: FanType::Geared,
            diameter_m,
            stage_count: 1,
            pressure_ratio: 1.45,
            mass_flow_kg_s,
            tip_speed_m_s: tip_speed,
            polytropic_efficiency: 0.93,
        }
    }

    pub fn low_bypass(diameter_m: f64, mass_flow_kg_s: f64) -> Self {
        Self {
            fan_type: FanType::Single,
            diameter_m,
            stage_count: 3,
            pressure_ratio: 4.5,
            mass_flow_kg_s,
            tip_speed_m_s: 480.0,
            polytropic_efficiency: 0.87,
        }
    }

    pub fn face_area_m2(&self) -> f64 {
        std::f64::consts::PI * (self.diameter_m / 2.0).powi(2)
    }
}
