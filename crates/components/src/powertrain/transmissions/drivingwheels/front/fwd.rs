#[derive(Debug, Clone)]
pub struct FrontWheelDrive {
    pub transverse_engine: bool,
    pub front_axle_max_torque_nm: f64,
    pub limited_slip: bool,
}

impl FrontWheelDrive {
    pub fn transverse(front_axle_max_torque_nm: f64) -> Self {
        Self { transverse_engine: true, front_axle_max_torque_nm, limited_slip: false }
    }

    pub fn with_lsd(front_axle_max_torque_nm: f64) -> Self {
        Self { transverse_engine: true, front_axle_max_torque_nm, limited_slip: true }
    }
}
