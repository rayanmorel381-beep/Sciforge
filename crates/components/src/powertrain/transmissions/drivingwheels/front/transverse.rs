#[derive(Debug, Clone)]
pub struct Transverse {
    pub front_axle_max_torque_nm: f64,
    pub limited_slip: bool,
}

impl Transverse {
    pub fn new(front_axle_max_torque_nm: f64) -> Self {
        Self { front_axle_max_torque_nm, limited_slip: false }
    }

    pub fn with_lsd(front_axle_max_torque_nm: f64) -> Self {
        Self { front_axle_max_torque_nm, limited_slip: true }
    }
}
