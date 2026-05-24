#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReverserType {
    CascadeBucket,
    PivotingDoor,
    TargetBucket,
    ColdStreamOnly,
}

#[derive(Debug, Clone)]
pub struct ThrustReverser {
    pub reverser_type: ReverserType,
    pub reversal_efficiency: f64,
    pub deploy_time_s: f64,
    pub max_reverse_thrust_fraction: f64,
    pub min_speed_kt: u16,
}

impl ThrustReverser {
    pub fn cascade_bucket(bypass_ratio: f64) -> Self {
        let efficiency = if bypass_ratio > 4.0 { 0.45 } else { 0.35 };
        Self {
            reverser_type: ReverserType::CascadeBucket,
            reversal_efficiency: efficiency,
            deploy_time_s: 1.5,
            max_reverse_thrust_fraction: 0.45,
            min_speed_kt: 60,
        }
    }

    pub fn pivoting_door() -> Self {
        Self {
            reverser_type: ReverserType::PivotingDoor,
            reversal_efficiency: 0.38,
            deploy_time_s: 1.2,
            max_reverse_thrust_fraction: 0.40,
            min_speed_kt: 70,
        }
    }

    pub fn target_bucket() -> Self {
        Self {
            reverser_type: ReverserType::TargetBucket,
            reversal_efficiency: 0.42,
            deploy_time_s: 1.8,
            max_reverse_thrust_fraction: 0.42,
            min_speed_kt: 55,
        }
    }

    pub fn reverse_thrust_kn(&self, forward_thrust_kn: f64) -> f64 {
        forward_thrust_kn * self.max_reverse_thrust_fraction * self.reversal_efficiency
    }
}
