#[derive(Debug, Clone)]
pub struct DoubleWishbone {
    pub upper_arm_length_mm: f64,
    pub lower_arm_length_mm: f64,
    pub camber_adjustment: bool,
    pub toe_adjustment: bool,
    pub aluminum_arms: bool,
}

impl DoubleWishbone {
    pub fn steel(upper_arm_length_mm: f64, lower_arm_length_mm: f64) -> Self {
        Self { upper_arm_length_mm, lower_arm_length_mm, camber_adjustment: false, toe_adjustment: false, aluminum_arms: false }
    }

    pub fn performance(upper_arm_length_mm: f64, lower_arm_length_mm: f64) -> Self {
        Self { upper_arm_length_mm, lower_arm_length_mm, camber_adjustment: true, toe_adjustment: true, aluminum_arms: true }
    }
}
