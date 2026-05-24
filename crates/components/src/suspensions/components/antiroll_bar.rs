#[derive(Debug, Clone)]
pub struct AntiRollBar {
    pub diameter_mm: f64,
    pub length_mm: f64,
    pub hollow: bool,
    pub adjustable: bool,
    pub drop_link_length_mm: f64,
}

impl AntiRollBar {
    pub fn solid(diameter_mm: f64, length_mm: f64, drop_link_length_mm: f64) -> Self {
        Self { diameter_mm, length_mm, hollow: false, adjustable: false, drop_link_length_mm }
    }

    pub fn hollow(diameter_mm: f64, length_mm: f64, drop_link_length_mm: f64) -> Self {
        Self { diameter_mm, length_mm, hollow: true, adjustable: false, drop_link_length_mm }
    }

    pub fn adjustable(diameter_mm: f64, length_mm: f64, drop_link_length_mm: f64) -> Self {
        Self { diameter_mm, length_mm, hollow: true, adjustable: true, drop_link_length_mm }
    }
}
