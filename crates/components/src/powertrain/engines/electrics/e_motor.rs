#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorTopology {
    Pmsm,
    Induction,
    AxialFlux,
    SwitchedReluctance,
}

#[derive(Debug, Clone)]
pub struct EMotor {
    pub topology: MotorTopology,
    pub peak_power_kw: f64,
    pub continuous_power_kw: f64,
    pub peak_torque_nm: f64,
    pub continuous_torque_nm: f64,
    pub base_speed_rpm: f64,
    pub max_speed_rpm: f64,
    pub nominal_voltage_v: f64,
    pub peak_current_a: f64,
    pub efficiency: f64,
    pub mass_kg: f64,
    pub stator_diameter_mm: f64,
    pub stack_length_mm: f64,
}

impl Default for EMotor {
    fn default() -> Self {
        Self::pmsm(150.0, 400.0)
    }
}

fn topology_specs(topology: MotorTopology) -> (f64, f64, f64, f64, f64) {
    match topology {
        MotorTopology::Pmsm => (0.95, 12000.0, 0.40, 1.50, 0.85),
        MotorTopology::Induction => (0.92, 16000.0, 0.55, 1.40, 0.70),
        MotorTopology::AxialFlux => (0.96, 10000.0, 0.25, 2.20, 0.92),
        MotorTopology::SwitchedReluctance => (0.91, 14000.0, 0.45, 1.30, 0.75),
    }
}

fn build(topology: MotorTopology, peak_power_kw: f64, nominal_voltage_v: f64) -> EMotor {
    let (efficiency, max_speed_rpm, mass_per_kw, torque_factor, continuous_ratio) =
        topology_specs(topology);
    let base_speed_rpm = max_speed_rpm * 0.35;
    let continuous_power_kw = peak_power_kw * continuous_ratio;
    let omega_base = base_speed_rpm * 2.0 * std::f64::consts::PI / 60.0;
    let peak_torque_nm = peak_power_kw * 1000.0 / omega_base * torque_factor;
    let continuous_torque_nm = peak_torque_nm * continuous_ratio;
    let peak_current_a = peak_power_kw * 1000.0 / (nominal_voltage_v * efficiency);
    let mass_kg = peak_power_kw * mass_per_kw + 8.0;
    let stator_diameter_mm = (140.0 + (peak_torque_nm / 50.0).sqrt() * 14.0).clamp(150.0, 320.0);
    let stack_length_mm = match topology {
        MotorTopology::AxialFlux => stator_diameter_mm * 0.35,
        _ => stator_diameter_mm * 0.85,
    };
    EMotor {
        topology,
        peak_power_kw,
        continuous_power_kw: (continuous_power_kw * 10.0).round() / 10.0,
        peak_torque_nm: (peak_torque_nm * 10.0).round() / 10.0,
        continuous_torque_nm: (continuous_torque_nm * 10.0).round() / 10.0,
        base_speed_rpm,
        max_speed_rpm,
        nominal_voltage_v,
        peak_current_a: (peak_current_a * 10.0).round() / 10.0,
        efficiency,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        stator_diameter_mm: (stator_diameter_mm * 10.0).round() / 10.0,
        stack_length_mm: (stack_length_mm * 10.0).round() / 10.0,
    }
}

impl EMotor {
    pub fn pmsm(peak_power_kw: f64, nominal_voltage_v: f64) -> Self {
        build(MotorTopology::Pmsm, peak_power_kw, nominal_voltage_v)
    }
    pub fn induction(peak_power_kw: f64, nominal_voltage_v: f64) -> Self {
        build(MotorTopology::Induction, peak_power_kw, nominal_voltage_v)
    }
    pub fn axial_flux(peak_power_kw: f64, nominal_voltage_v: f64) -> Self {
        build(MotorTopology::AxialFlux, peak_power_kw, nominal_voltage_v)
    }
    pub fn switched_reluctance(peak_power_kw: f64, nominal_voltage_v: f64) -> Self {
        build(MotorTopology::SwitchedReluctance, peak_power_kw, nominal_voltage_v)
    }

    pub fn peak_losses_kw(&self) -> f64 {
        self.peak_power_kw * (1.0 - self.efficiency)
    }
}
