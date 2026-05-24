#[derive(Debug, Clone)]
pub struct DeDion {
    pub rear_axle_max_torque_nm: f64,
    pub differential_inboard: bool,
}

impl DeDion {
    pub fn new(rear_axle_max_torque_nm: f64) -> Self {
        Self { rear_axle_max_torque_nm, differential_inboard: true }
    }
}
