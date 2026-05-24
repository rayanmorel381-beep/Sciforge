#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgniterType {
    SurfaceDischarge,
    HighEnergy,
    PlasmaJet,
}

#[derive(Debug, Clone)]
pub struct TurbineIgnition {
    pub igniter_type: IgniterType,
    pub exciter_voltage_v: u16,
    pub energy_per_spark_j: f64,
    pub spark_rate_hz: f64,
    pub igniter_count: u8,
    pub continuous_relight: bool,
}

impl TurbineIgnition {
    pub fn surface_discharge(igniter_count: u8) -> Self {
        Self {
            igniter_type: IgniterType::SurfaceDischarge,
            exciter_voltage_v: 2_000,
            energy_per_spark_j: 4.0,
            spark_rate_hz: 1.0,
            igniter_count,
            continuous_relight: false,
        }
    }

    pub fn high_energy(igniter_count: u8) -> Self {
        Self {
            igniter_type: IgniterType::HighEnergy,
            exciter_voltage_v: 20_000,
            energy_per_spark_j: 12.0,
            spark_rate_hz: 4.0,
            igniter_count,
            continuous_relight: true,
        }
    }

    pub fn plasma_jet(igniter_count: u8) -> Self {
        Self {
            igniter_type: IgniterType::PlasmaJet,
            exciter_voltage_v: 30_000,
            energy_per_spark_j: 25.0,
            spark_rate_hz: 8.0,
            igniter_count,
            continuous_relight: true,
        }
    }

    pub fn total_power_w(&self) -> f64 {
        self.energy_per_spark_j * self.spark_rate_hz * self.igniter_count as f64
    }
}
