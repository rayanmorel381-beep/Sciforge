#[derive(Debug, Clone)]
pub struct FullTime {
    pub front_torque_split_pct: u8,
    pub rear_torque_split_pct: u8,
    pub centre_differential: bool,
    pub torque_vectoring: bool,
}

impl FullTime {
    pub fn new(front_pct: u8) -> Self {
        Self {
            front_torque_split_pct: front_pct,
            rear_torque_split_pct: 100 - front_pct,
            centre_differential: true,
            torque_vectoring: false,
        }
    }

    pub fn with_torque_vectoring(front_pct: u8) -> Self {
        Self {
            front_torque_split_pct: front_pct,
            rear_torque_split_pct: 100 - front_pct,
            centre_differential: true,
            torque_vectoring: true,
        }
    }
}
