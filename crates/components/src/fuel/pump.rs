#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PumpType {
    Mechanical,
    Electric,
    HighPressureDirect,
    ImmersedElectric,
    LowPressureTransfer,
}

#[derive(Debug, Clone)]
pub struct FuelPump {
    pub pump_type: PumpType,
    pub flow_l_h: f64,
    pub pressure_bar: f64,
    pub voltage_v: f64,
    pub regulated: bool,
}

impl FuelPump {
    pub fn mechanical(flow_l_h: f64) -> Self {
        Self { pump_type: PumpType::Mechanical, flow_l_h, pressure_bar: 0.4, voltage_v: 0.0, regulated: false }
    }

    pub fn electric(flow_l_h: f64, pressure_bar: f64) -> Self {
        Self { pump_type: PumpType::Electric, flow_l_h, pressure_bar, voltage_v: 12.0, regulated: true }
    }

    pub fn high_pressure_direct(flow_l_h: f64) -> Self {
        Self { pump_type: PumpType::HighPressureDirect, flow_l_h, pressure_bar: 200.0, voltage_v: 12.0, regulated: true }
    }

    pub fn immersed(flow_l_h: f64, pressure_bar: f64) -> Self {
        Self { pump_type: PumpType::ImmersedElectric, flow_l_h, pressure_bar, voltage_v: 12.0, regulated: true }
    }

    pub fn transfer(flow_l_h: f64) -> Self {
        Self { pump_type: PumpType::LowPressureTransfer, flow_l_h, pressure_bar: 0.2, voltage_v: 24.0, regulated: false }
    }
}
