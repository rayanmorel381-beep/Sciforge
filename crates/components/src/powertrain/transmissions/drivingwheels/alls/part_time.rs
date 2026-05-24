#[derive(Debug, Clone)]
pub struct PartTime {
    pub front_torque_split_pct: u8,
    pub rear_torque_split_pct: u8,
    pub transfer_case: bool,
    pub low_range: bool,
}

impl PartTime {
    pub fn new(front_pct: u8) -> Self {
        Self {
            front_torque_split_pct: front_pct,
            rear_torque_split_pct: 100 - front_pct,
            transfer_case: true,
            low_range: false,
        }
    }

    pub fn with_low_range(front_pct: u8) -> Self {
        Self {
            front_torque_split_pct: front_pct,
            rear_torque_split_pct: 100 - front_pct,
            transfer_case: true,
            low_range: true,
        }
    }
}
