use crate::powertrain::transmissions::assemblies::manuals::ManualGearbox;
use crate::powertrain::transmissions::assemblies::GearboxFailureMode;

#[derive(Debug, Clone)]
pub struct RobotizedManual {
    pub gearbox: ManualGearbox,
    pub shift_time_ms: u32,
    pub actuator_response_ms: u32,
    pub actuator_pressure_pa: f64,
    pub actuator_piston_area_m2: f64,
    pub actuator_force_n: f64,
    pub actuator_cycles: u64,
    pub actuator_wear: f64,
    pub actuator_failed: bool,
}

impl RobotizedManual {
    pub fn code(&self) -> String {
        format!("AMT{}", &self.gearbox.code()[3..])
    }

    pub fn ratio(&self, gear: u8) -> f64 {
        self.gearbox.ratio(gear)
    }

    pub fn shift(&mut self, from_gear: u8, to_gear: u8, rpm_in: f64) {
        if self.actuator_failed {
            return;
        }
        self.actuator_cycles += 1;
        self.actuator_wear += 1.0 / 8.0e5;
        if self.actuator_wear > 1.0 {
            self.actuator_failed = true;
            if !self.gearbox.failures.contains(&GearboxFailureMode::ShiftForkBent) {
                self.gearbox.failures.push(GearboxFailureMode::ShiftForkBent);
            }
            return;
        }
        let energy_j = self.gearbox.shift_energy_j(from_gear, to_gear, rpm_in);
        let synchro_capacity_j = self.gearbox.max_input_torque_nm * 50.0;
        if synchro_capacity_j > 0.0 {
            self.gearbox.wear_factor += energy_j / (synchro_capacity_j * 1.0e5);
        }
    }

    pub fn tick(
        &mut self,
        rpm_in: f64,
        torque_in_nm: f64,
        gear: u8,
        ambient_k: f64,
        dt_s: f64,
    ) {
        self.gearbox.tick(rpm_in, torque_in_nm, gear, ambient_k, dt_s);
    }

    pub fn current_efficiency(&self) -> f64 {
        if self.actuator_failed {
            return 0.0;
        }
        self.gearbox.current_efficiency()
    }

    pub fn is_running(&self) -> bool {
        !self.actuator_failed && self.gearbox.is_running()
    }
}

pub fn with_latency(gearbox: ManualGearbox, disp_l: f64) -> RobotizedManual {
    let shift_time_ms = if disp_l < 1.0 { 520 }
        else if disp_l < 1.6 { 480 }
        else if disp_l < 2.2 { 440 }
        else if disp_l < 3.0 { 400 }
        else if disp_l < 4.5 { 360 }
        else if disp_l < 6.5 { 320 }
        else if disp_l < 10.0 { 290 }
        else { 260 };
    let actuator_response_ms = (shift_time_ms / 4).max(40);
    let actuator_pressure_pa = 60.0e5;
    let piston_diameter_m = 0.024 + (disp_l / 12.0).min(0.020);
    let piston_area_m2 = std::f64::consts::PI * (piston_diameter_m * 0.5).powi(2);
    let actuator_force_n = actuator_pressure_pa * piston_area_m2;
    RobotizedManual {
        gearbox,
        shift_time_ms,
        actuator_response_ms,
        actuator_pressure_pa,
        actuator_piston_area_m2: piston_area_m2,
        actuator_force_n,
        actuator_cycles: 0,
        actuator_wear: 0.0,
        actuator_failed: false,
    }
}
