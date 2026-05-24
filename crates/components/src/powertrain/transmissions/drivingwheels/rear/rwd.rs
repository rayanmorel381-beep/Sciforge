#[derive(Debug, Clone)]
pub struct RearWheelDrive {
    pub rear_axle_max_torque_nm: f64,
    pub limited_slip: bool,
    pub torque_vectoring: bool,
}

impl RearWheelDrive {
    pub fn open(rear_axle_max_torque_nm: f64) -> Self {
        Self { rear_axle_max_torque_nm, limited_slip: false, torque_vectoring: false }
    }

    pub fn with_lsd(rear_axle_max_torque_nm: f64) -> Self {
        Self { rear_axle_max_torque_nm, limited_slip: true, torque_vectoring: false }
    }

    pub fn with_torque_vectoring(rear_axle_max_torque_nm: f64) -> Self {
        Self { rear_axle_max_torque_nm, limited_slip: true, torque_vectoring: true }
    }
}
