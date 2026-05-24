use super::system::AwdSystem;

#[derive(Debug, Clone)]
pub struct AllWheelDrive {
    pub system: AwdSystem,
    pub front_torque_split_pct: u8,
    pub rear_torque_split_pct: u8,
    pub centre_differential: bool,
    pub torque_vectoring: bool,
}

impl AllWheelDrive {
    pub fn full_time(front_pct: u8) -> Self {
        Self {
            system: AwdSystem::FullTime,
            front_torque_split_pct: front_pct,
            rear_torque_split_pct: 100 - front_pct,
            centre_differential: true,
            torque_vectoring: false,
        }
    }

    pub fn on_demand() -> Self {
        Self {
            system: AwdSystem::OnDemand,
            front_torque_split_pct: 0,
            rear_torque_split_pct: 100,
            centre_differential: false,
            torque_vectoring: false,
        }
    }

    pub fn with_torque_vectoring(front_pct: u8) -> Self {
        Self {
            system: AwdSystem::FullTime,
            front_torque_split_pct: front_pct,
            rear_torque_split_pct: 100 - front_pct,
            centre_differential: true,
            torque_vectoring: true,
        }
    }
}
