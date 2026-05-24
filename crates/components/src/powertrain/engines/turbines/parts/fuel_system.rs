#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvFuelSystemType {
    LowPressure,
    HighPressure,
    DualFeed,
}

#[derive(Debug, Clone)]
pub struct AvFuelSystem {
    pub system_type: AvFuelSystemType,
    pub pump_pressure_bar: f64,
    pub flow_capacity_kg_h: f64,
    pub fadec: bool,
    pub fuel_filter_micron: u8,
}

impl AvFuelSystem {
    pub fn low_pressure(flow_capacity_kg_h: f64) -> Self {
        Self {
            system_type: AvFuelSystemType::LowPressure,
            pump_pressure_bar: 40.0,
            flow_capacity_kg_h,
            fadec: false,
            fuel_filter_micron: 25,
        }
    }

    pub fn high_pressure(flow_capacity_kg_h: f64) -> Self {
        Self {
            system_type: AvFuelSystemType::HighPressure,
            pump_pressure_bar: 120.0,
            flow_capacity_kg_h,
            fadec: true,
            fuel_filter_micron: 10,
        }
    }

    pub fn dual_feed(flow_capacity_kg_h: f64) -> Self {
        Self {
            system_type: AvFuelSystemType::DualFeed,
            pump_pressure_bar: 100.0,
            flow_capacity_kg_h,
            fadec: true,
            fuel_filter_micron: 10,
        }
    }
}
