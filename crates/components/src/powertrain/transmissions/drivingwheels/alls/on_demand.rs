#[derive(Debug, Clone)]
pub struct OnDemand {
    pub default_rear_bias: bool,
    pub max_front_torque_split_pct: u8,
    pub hydraulic_coupling: bool,
}

impl OnDemand {
    pub fn rear_biased(max_front_pct: u8) -> Self {
        Self { default_rear_bias: true, max_front_torque_split_pct: max_front_pct, hydraulic_coupling: true }
    }

    pub fn symmetric(max_front_pct: u8) -> Self {
        Self { default_rear_bias: false, max_front_torque_split_pct: max_front_pct, hydraulic_coupling: false }
    }
}
