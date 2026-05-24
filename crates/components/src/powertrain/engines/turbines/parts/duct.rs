#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuctType {
    BypassDuct,
    InterTurbineDuct,
    TransitionDuct,
    SChannel,
    IntakeDuct,
}

#[derive(Debug, Clone)]
pub struct Duct {
    pub duct_type: DuctType,
    pub length_m: f64,
    pub inlet_diameter_m: f64,
    pub exit_diameter_m: f64,
    pub pressure_loss_fraction: f64,
    pub insulated: bool,
}

impl Duct {
    pub fn bypass(length_m: f64, fan_diameter_m: f64) -> Self {
        Self {
            duct_type: DuctType::BypassDuct,
            length_m,
            inlet_diameter_m: fan_diameter_m,
            exit_diameter_m: fan_diameter_m * 0.95,
            pressure_loss_fraction: 0.008,
            insulated: false,
        }
    }

    pub fn inter_turbine(inlet_diameter_m: f64, exit_diameter_m: f64) -> Self {
        Self {
            duct_type: DuctType::InterTurbineDuct,
            length_m: 0.25,
            inlet_diameter_m,
            exit_diameter_m,
            pressure_loss_fraction: 0.015,
            insulated: true,
        }
    }

    pub fn s_channel(length_m: f64, inlet_diameter_m: f64, exit_diameter_m: f64) -> Self {
        Self {
            duct_type: DuctType::SChannel,
            length_m,
            inlet_diameter_m,
            exit_diameter_m,
            pressure_loss_fraction: 0.020,
            insulated: false,
        }
    }

    pub fn area_ratio(&self) -> f64 {
        let a_exit = std::f64::consts::PI * (self.exit_diameter_m / 2.0).powi(2);
        let a_inlet = std::f64::consts::PI * (self.inlet_diameter_m / 2.0).powi(2);
        if a_inlet > 0.0 { a_exit / a_inlet } else { 1.0 }
    }
}
