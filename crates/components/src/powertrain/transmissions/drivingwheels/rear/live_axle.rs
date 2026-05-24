#[derive(Debug, Clone)]
pub struct LiveAxle {
    pub rear_axle_max_torque_nm: f64,
    pub track_mm: f64,
    pub watts_linkage: bool,
}

impl LiveAxle {
    pub fn new(rear_axle_max_torque_nm: f64, track_mm: f64) -> Self {
        Self { rear_axle_max_torque_nm, track_mm, watts_linkage: false }
    }

    pub fn with_watts(rear_axle_max_torque_nm: f64, track_mm: f64) -> Self {
        Self { rear_axle_max_torque_nm, track_mm, watts_linkage: true }
    }
}
