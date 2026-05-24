#[derive(Debug, Clone)]
pub struct OilCooler {
    pub rows: u8,
    pub area_cm2: f64,
    pub thermostatic: bool,
}

impl OilCooler {
    pub fn standard() -> Self {
        Self { rows: 13, area_cm2: 360.0, thermostatic: true }
    }

    pub fn performance() -> Self {
        Self { rows: 19, area_cm2: 520.0, thermostatic: true }
    }
}
