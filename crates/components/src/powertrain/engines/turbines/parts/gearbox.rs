#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GearboxType {
    PropellerReduction,
    AccessoryDrive,
    PowerTakeoff,
}

#[derive(Debug, Clone)]
pub struct TurbineGearbox {
    pub gearbox_type: GearboxType,
    pub reduction_ratio: f64,
    pub max_power_kw: f64,
    pub efficiency: f64,
    pub input_rpm: u32,
    pub output_rpm: u32,
}

impl TurbineGearbox {
    pub fn propeller_reduction(max_power_kw: f64, reduction_ratio: f64, input_rpm: u32) -> Self {
        let output_rpm = (input_rpm as f64 / reduction_ratio) as u32;
        Self {
            gearbox_type: GearboxType::PropellerReduction,
            reduction_ratio,
            max_power_kw,
            efficiency: 0.987,
            input_rpm,
            output_rpm,
        }
    }

    pub fn accessory_drive(input_rpm: u32) -> Self {
        Self {
            gearbox_type: GearboxType::AccessoryDrive,
            reduction_ratio: 3.0,
            max_power_kw: 80.0,
            efficiency: 0.98,
            input_rpm,
            output_rpm: input_rpm / 3,
        }
    }

    pub fn power_takeoff(max_power_kw: f64, input_rpm: u32, reduction_ratio: f64) -> Self {
        let output_rpm = (input_rpm as f64 / reduction_ratio) as u32;
        Self {
            gearbox_type: GearboxType::PowerTakeoff,
            reduction_ratio,
            max_power_kw,
            efficiency: 0.985,
            input_rpm,
            output_rpm,
        }
    }

    pub fn output_torque_nm(&self) -> f64 {
        let power_w = self.max_power_kw * 1_000.0 * self.efficiency;
        let omega = self.output_rpm as f64 * 2.0 * std::f64::consts::PI / 60.0;
        if omega > 0.0 { power_w / omega } else { 0.0 }
    }
}
