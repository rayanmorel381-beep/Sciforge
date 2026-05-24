#[derive(Debug, Clone)]
pub struct TrailingArm {
    pub arm_length_mm: f64,
    pub twist_beam: bool,
    pub panhard_rod: bool,
}

impl TrailingArm {
    pub fn simple(arm_length_mm: f64) -> Self {
        Self { arm_length_mm, twist_beam: false, panhard_rod: false }
    }

    pub fn with_panhard(arm_length_mm: f64) -> Self {
        Self { arm_length_mm, twist_beam: false, panhard_rod: true }
    }
}
