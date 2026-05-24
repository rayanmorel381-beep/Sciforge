#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShoeLayout {
    LeadingTrailing,
    DuplexLeading,
    Servo,
}

#[derive(Debug, Clone)]
pub struct BrakeShoe {
    pub layout: ShoeLayout,
    pub lining_width_mm: f64,
    pub lining_thickness_mm: f64,
    pub friction_coefficient: f64,
}

impl BrakeShoe {
    pub fn leading_trailing(lining_width_mm: f64) -> Self {
        Self { layout: ShoeLayout::LeadingTrailing, lining_width_mm, lining_thickness_mm: 5.0, friction_coefficient: 0.38 }
    }

    pub fn duplex(lining_width_mm: f64) -> Self {
        Self { layout: ShoeLayout::DuplexLeading, lining_width_mm, lining_thickness_mm: 5.0, friction_coefficient: 0.42 }
    }

    pub fn servo(lining_width_mm: f64) -> Self {
        Self { layout: ShoeLayout::Servo, lining_width_mm, lining_thickness_mm: 5.0, friction_coefficient: 0.45 }
    }
}
