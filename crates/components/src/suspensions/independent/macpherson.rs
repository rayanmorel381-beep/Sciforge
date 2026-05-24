#[derive(Debug, Clone)]
pub struct MacPherson {
    pub strut_length_mm: f64,
    pub lower_arm_length_mm: f64,
    pub camber_angle_deg: f64,
    pub aluminum_strut: bool,
}

impl MacPherson {
    pub fn new(strut_length_mm: f64, lower_arm_length_mm: f64, camber_angle_deg: f64) -> Self {
        Self { strut_length_mm, lower_arm_length_mm, camber_angle_deg, aluminum_strut: false }
    }

    pub fn aluminum(strut_length_mm: f64, lower_arm_length_mm: f64, camber_angle_deg: f64) -> Self {
        Self { strut_length_mm, lower_arm_length_mm, camber_angle_deg, aluminum_strut: true }
    }
}
