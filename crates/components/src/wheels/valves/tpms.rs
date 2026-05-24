#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpmsProtocol {
    Rf315Mhz,
    Rf433Mhz,
    Bluetooth,
    Lf125Khz,
}

#[derive(Debug, Clone)]
pub struct TpmsValve {
    pub protocol: TpmsProtocol,
    pub thread_diameter_mm: f64,
    pub max_pressure_bar: f64,
    pub alert_delta_bar: f64,
    pub battery_life_years: u8,
    pub direct: bool,
    pub temperature_sensor: bool,
}

impl TpmsValve {
    pub fn direct_rf433(thread_diameter_mm: f64) -> Self {
        Self {
            protocol: TpmsProtocol::Rf433Mhz,
            thread_diameter_mm,
            max_pressure_bar: 10.0,
            alert_delta_bar: 0.2,
            battery_life_years: 7,
            direct: true,
            temperature_sensor: false,
        }
    }

    pub fn direct_with_temp(thread_diameter_mm: f64) -> Self {
        Self {
            protocol: TpmsProtocol::Rf433Mhz,
            thread_diameter_mm,
            max_pressure_bar: 10.0,
            alert_delta_bar: 0.2,
            battery_life_years: 5,
            direct: true,
            temperature_sensor: true,
        }
    }

    pub fn indirect(thread_diameter_mm: f64) -> Self {
        Self {
            protocol: TpmsProtocol::Lf125Khz,
            thread_diameter_mm,
            max_pressure_bar: 10.0,
            alert_delta_bar: 0.3,
            battery_life_years: 0,
            direct: false,
            temperature_sensor: false,
        }
    }
}
