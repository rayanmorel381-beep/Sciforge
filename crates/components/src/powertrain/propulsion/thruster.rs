#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThrusterType {
    Chemical,
    IonElectrostatic,
    HallEffect,
    ColdGas,
    Arcjet,
    WaterJet,
    AzimuthPod,
}

#[derive(Debug, Clone)]
pub struct Thruster {
    pub thruster_type: ThrusterType,
    pub thrust_n: f64,
    pub specific_impulse_s: f64,
}

impl Thruster {
    pub fn chemical(thrust_n: f64, specific_impulse_s: f64) -> Self {
        Self { thruster_type: ThrusterType::Chemical, thrust_n, specific_impulse_s }
    }

    pub fn ion(thrust_n: f64, specific_impulse_s: f64) -> Self {
        Self { thruster_type: ThrusterType::IonElectrostatic, thrust_n, specific_impulse_s }
    }

    pub fn hall_effect(thrust_n: f64, specific_impulse_s: f64) -> Self {
        Self { thruster_type: ThrusterType::HallEffect, thrust_n, specific_impulse_s }
    }

    pub fn water_jet(thrust_n: f64) -> Self {
        Self { thruster_type: ThrusterType::WaterJet, thrust_n, specific_impulse_s: 0.0 }
    }

    pub fn azimuth_pod(thrust_n: f64) -> Self {
        Self { thruster_type: ThrusterType::AzimuthPod, thrust_n, specific_impulse_s: 0.0 }
    }
}
