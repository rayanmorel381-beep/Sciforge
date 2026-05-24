#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PintleType {
    Military,
    Commercial,
    Agricultural,
}

#[derive(Debug, Clone)]
pub struct PintleHook {
    pub pintle_type: PintleType,
    pub max_vertical_load_kg: f64,
    pub max_drawbar_pull_kg: f64,
}

impl PintleHook {
    pub fn new(pintle_type: PintleType, max_vertical_load_kg: f64, max_drawbar_pull_kg: f64) -> Self {
        Self { pintle_type, max_vertical_load_kg, max_drawbar_pull_kg }
    }
}
